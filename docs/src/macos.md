---
title: Kurocto on macOS
description: "Kurocto is developed primarily on macOS, making it a first-class platform with full feature support."
---

# Kurocto on macOS

Kurocto is developed primarily on macOS, making it a first-class platform with full feature support.

## Installing Kurocto

Kurocto does not have published builds, a download site, or a Homebrew package yet. It is currently built from source; see the [macOS development documentation](./development/macos.md).

## System Requirements

- macOS 10.15.7 (Catalina) or later
- Apple Silicon (M1/M2/M3/M4) or Intel processor

Kurocto uses Metal for GPU-accelerated rendering, which is available on all supported macOS versions.

## Installing the CLI

Kurocto includes a command-line tool for opening files and projects from Terminal. To install it:

1. Open Kurocto
2. Open the command palette with `Cmd+Shift+P`
3. Run {#action cli::InstallCliBinary}

This creates a `kurocto` command in `/usr/local/bin`. You can then open files and folders:

```sh
kurocto .                    # Open current folder
kurocto file.txt             # Open a file
kurocto project/ file.txt    # Open a folder and a file
```

See the [CLI Reference](./reference/cli.md) for all available options.

## Uninstall

1. Quit Kurocto if it's running
2. Drag Kurocto from Applications to the Trash
3. Optionally, remove your settings and extensions:

```sh
rm -rf ~/.config/kurocto
rm -rf ~/Library/Application\ Support/Kurocto
rm -rf ~/Library/Caches/Kurocto
rm -rf ~/Library/Logs/Kurocto
rm -rf ~/Library/Saved\ Application\ State/dev.kurocto.Kurocto.savedState
```

If you installed the CLI, remove it with:

```sh
rm /usr/local/bin/kurocto
```

## Troubleshooting

### Kurocto won't open or shows "damaged" warning

If macOS reports that Kurocto is damaged or can't be opened, it's likely a Gatekeeper issue. Try:

1. Right-click (or Control-click) on Kurocto in Applications
2. Select "Open" from the context menu
3. Click "Open" in the dialog that appears

This tells macOS to trust the application.

If that doesn't work, remove the quarantine attribute:

```sh
xattr -cr /Applications/Kurocto.app
```

### CLI command not found

If the `kurocto` command isn't available after installation:

1. Check that `/usr/local/bin` is in your PATH
2. Try reinstalling the CLI via {#action cli::InstallCliBinary} in the command palette
3. Open a new terminal window to reload your PATH

### Can't install CLI {#cant-install-cli}

{#action cli::InstallCliBinary} writes a `kurocto` symlink to `/usr/local/bin`, which requires administrator privileges. If your macOS account isn't in the `admin` group, Kurocto can't create that symlink and will report that it can't install the CLI automatically.

Instead, you can add an alias pointing to the `cli` binary bundled inside the app. The path depends on where Kurocto is installed:

```sh
# Default install (Kurocto in /Applications)
alias kurocto="/Applications/Kurocto.app/Contents/MacOS/cli"

# User install (Kurocto in ~/Applications)
alias kurocto="$HOME/Applications/Kurocto.app/Contents/MacOS/cli"

# Preview build (Kurocto Preview in ~/Applications)
alias kurocto="$HOME/Applications/Kurocto Preview.app/Contents/MacOS/cli"
```

Add the line that matches your install to your shell configuration file. Use `~/.zshrc` for Zsh (the default on modern macOS) or `~/.bashrc` for Bash.

After you restart your shell, you will be able to use `kurocto` from your terminal:

```sh
kurocto .              # Open current folder
kurocto file.txt       # Open a file
```

### GPU or rendering issues

Kurocto uses Metal for rendering. If you experience graphical glitches:

1. Ensure macOS is up to date
2. Restart your Mac to reset the GPU state
3. Check Activity Monitor for GPU pressure from other apps

### High memory or CPU usage

If Kurocto uses more resources than expected:

1. Check for runaway language servers in the terminal output ({#action zed::OpenLog})
2. Try disabling extensions one by one to identify conflicts
3. For large projects, consider using [project settings](./reference/all-settings.md#file-scan-exclusions) to exclude unnecessary folders from indexing

For additional help, see the [Troubleshooting guide](./troubleshooting.md).
