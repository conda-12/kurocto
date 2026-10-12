[CmdletBinding()]
Param(
    [Parameter()][Alias('i')][switch]$Install,
    [Parameter()][Alias('h')][switch]$Help,
    [Parameter()][Alias('a')][string]$Architecture,
    [Parameter()][string]$Name
)

. "$PSScriptRoot/lib/workspace.ps1"

# https://stackoverflow.com/questions/57949031/powershell-script-stops-if-program-fails-like-bash-set-o-errexit
$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$buildSuccess = $false
$canCodeSign = $false

$OSArchitecture = switch ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture) {
    "X64" { "x86_64" }
    "Arm64" { "aarch64" }
    default { throw "Unsupported architecture" }
}

$Architecture = if ($Architecture) {
    $Architecture
} else {
    $OSArchitecture
}

$CargoOutDir = "./target/$Architecture-pc-windows-msvc/release"

function Get-VSArch {
    param(
        [string]$Arch
    )

    switch ($Arch) {
        "x86_64" { "amd64" }
        "aarch64" { "arm64" }
    }
}

Push-Location
& "C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\Tools\Launch-VsDevShell.ps1" -Arch (Get-VSArch -Arch $Architecture) -HostArch (Get-VSArch -Arch $OSArchitecture)
Pop-Location

$target = "$Architecture-pc-windows-msvc"

if ($Help) {
    Write-Output "Usage: test.ps1 [-Install] [-Help]"
    Write-Output "Build the installer for Windows.\n"
    Write-Output "Options:"
    Write-Output "  -Architecture, -a Which architecture to build (x86_64 or aarch64)"
    Write-Output "  -Install, -i      Run the installer after building."
    Write-Output "  -Help, -h         Show this help message."
    exit 0
}

Push-Location -Path crates/zed
$channel = Get-Content "RELEASE_CHANNEL"
$env:ZED_RELEASE_CHANNEL = $channel
$env:RELEASE_CHANNEL = $channel
Pop-Location

function CheckEnvironmentVariables {
    if(-not $env:CI) {
        return
    }

    $requiredVars = @('ZED_WORKSPACE', 'RELEASE_VERSION', 'ZED_RELEASE_CHANNEL')

    foreach ($var in $requiredVars) {
        if ([string]::IsNullOrWhiteSpace([Environment]::GetEnvironmentVariable($var))) {
            Write-Error "$var is not set"
            exit 1
        }
    }

    # On PRs from forks the signing secrets are not populated,
    # so skip code signing instead of failing, like bundle-mac does.
    $signingVars = @(
        'AZURE_TENANT_ID', 'AZURE_CLIENT_ID', 'AZURE_CLIENT_SECRET',
        'ACCOUNT_NAME', 'CERT_PROFILE_NAME', 'ENDPOINT',
        'FILE_DIGEST', 'TIMESTAMP_DIGEST', 'TIMESTAMP_SERVER'
    )

    $missingVars = @($signingVars | Where-Object { [string]::IsNullOrWhiteSpace([Environment]::GetEnvironmentVariable($_)) })
    if ($missingVars.Count -eq 0) {
        $script:canCodeSign = $true
    } else {
        Write-Host "====== WARNING ======"
        Write-Host "One or more of the following variables are missing: $($missingVars -join ', ')"
        Write-Host "This bundle will not be code signed"
        Write-Host "====== WARNING ======"
    }
}

function PrepareForBundle {
    if (Test-Path "$innoDir") {
        Remove-Item -Path "$innoDir" -Recurse -Force
    }
    New-Item -Path "$innoDir" -ItemType Directory -Force
    Copy-Item -Path "$env:ZED_WORKSPACE\crates\zed\resources\windows\*" -Destination "$innoDir" -Recurse -Force
    New-Item -Path "$innoDir\make_appx" -ItemType Directory -Force
    New-Item -Path "$innoDir\appx" -ItemType Directory -Force
    New-Item -Path "$innoDir\bin" -ItemType Directory -Force

    rustup target add $target
}

function GenerateLicenses {
    . $PSScriptRoot/generate-licenses.ps1
    New-Item -Path "$innoDir\licenses" -ItemType Directory -Force
    Copy-Item -Path "$env:ZED_WORKSPACE\LICENSE-GPL", "$env:ZED_WORKSPACE\LICENSE-APACHE", "$env:ZED_WORKSPACE\assets\licenses.md" -Destination "$innoDir\licenses" -Force
}

function BuildZedAndItsFriends {
    Write-Output "Building Zed and its friends, for channel: $channel"
    # Build zed.exe and cli.exe
    cargo --config .cargo/bundle-config.toml build --release --package zed --package cli --target $target
    Copy-Item -Path ".\$CargoOutDir\zed.exe" -Destination "$innoDir\Zed.exe" -Force
    Copy-Item -Path ".\$CargoOutDir\cli.exe" -Destination "$innoDir\cli.exe" -Force
    # Build explorer_command_injector.dll
    switch ($channel) {
        "stable" {
            cargo --config .cargo/bundle-config.toml build --release --features stable --no-default-features --package explorer_command_injector --target $target
        }
        "preview" {
            cargo --config .cargo/bundle-config.toml build --release --features preview --no-default-features --package explorer_command_injector --target $target
        }
        default {
            cargo --config .cargo/bundle-config.toml build --release --package explorer_command_injector --target $target
        }
    }
    Copy-Item -Path ".\$CargoOutDir\explorer_command_injector.dll" -Destination "$innoDir\zed_explorer_command_injector.dll" -Force
}

function BuildRemoteServer {
    Write-Output "Building remote_server for $target"
    cargo --config .cargo/bundle-config.toml build --release --package remote_server --target $target

    # Create zipped remote server binary
    $remoteServerSrc = (Resolve-Path ".\$CargoOutDir\remote_server.exe").Path

    if ($canCodeSign) {
        Write-Output "Code signing remote_server.exe"
        & "$innoDir\sign.ps1" $remoteServerSrc
    }

    $remoteServerDst = "$env:ZED_WORKSPACE\target\zed-remote-server-windows-$Architecture.zip"
    Write-Output "Compressing remote_server to $remoteServerDst"
    Compress-Archive -Path $remoteServerSrc -DestinationPath $remoteServerDst -Force

    Write-Output "Remote server compressed successfully"
}

function ZipZedAndItsFriendsDebug {
    $items = @(
        ".\$CargoOutDir\zed.pdb",
        ".\$CargoOutDir\cli.pdb",
        ".\$CargoOutDir\explorer_command_injector.pdb",
        ".\$CargoOutDir\remote_server.pdb"
    )

    Compress-Archive -Path $items -DestinationPath ".\$CargoOutDir\zed-$env:RELEASE_VERSION-$env:ZED_RELEASE_CHANNEL.dbg.zip" -Force
}


DownloadConpty
CollectFiles
BuildInstaller

if ($buildSuccess) {
    Write-Output "Build successful"
    if ($Install) {
        Write-Output "Installing Zed..."
        Start-Process -FilePath "$env:ZED_WORKSPACE/target/ZedEditorUserSetup-x64-$env:RELEASE_VERSION.exe"
    }
    exit 0
}
else {
    Write-Output "Build failed"
    exit 1
}
