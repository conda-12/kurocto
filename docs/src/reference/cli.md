---
title: CLI Reference
description: "Reference for Kurocto's command-line interface (CLI), including opening files and directories, integrating with tools, and controlling Kurocto from scripts."
---

# CLI Reference

Use Kurocto's command-line interface (CLI) to open files and directories, integrate with other tools, and control Kurocto from scripts.

## Installation

**macOS:** Run the {#action cli::InstallCliBinary} command from the command palette ({#kb command_palette::Toggle}) to install the `kurocto` CLI to `/usr/local/bin/kurocto`.

**Linux:** The CLI is included with Kurocto packages. The CLI binary is named `kurocto`.

**Windows:** The CLI is included with Kurocto. Add Kurocto's installation directory to your PATH, or use the full path to `kurocto.exe`.

## Usage

```sh
kurocto [OPTIONS] [PATHS]...
```

## Opening Files and Directories

Open a file:

```sh
kurocto myfile.txt
```

Open a directory as a workspace:

```sh
kurocto ~/projects/myproject
```

Open multiple files or directories:

```sh
kurocto file1.txt file2.txt ~/projects/myproject
```

Open a file at a specific line and column:

```sh
kurocto myfile.txt:42        # Open at line 42
kurocto myfile.txt:42:10     # Open at line 42, column 10
```

## Options

### `-w`, `--wait`

Wait for all opened files to be closed before the CLI exits. When opening a directory, waits until the window is closed.

This is useful for integrating Kurocto with tools that expect an editor to block until editing is complete (e.g., `git commit`):

```sh
export EDITOR="kurocto --wait"
git commit  # Opens Kurocto and waits for you to close the commit message file
```

### `-n`, `--new`

Open paths in a new workspace window, even if the paths are already open in an existing window:

```sh
kurocto -n ~/projects/myproject
```

### `-a`, `--add`

Add paths to the currently focused workspace instead of opening a new window. When multiple workspace windows are open, files open in the focused window:

```sh
kurocto -a newfile.txt
```

### `-r`, `--reuse`

Reuse an existing window, replacing its current workspace with the new paths:

```sh
kurocto -r ~/projects/different-project
```

### `-e`, `--existing`

Open paths in an existing Kurocto window instead of creating a new one:

```sh
kurocto -e myfile.txt
```

By default (without `-n`, `-a`, `-r`, or `-e`), directories open in the current window's sidebar. You can change this default with the `cli_default_open_behavior` setting. See [Windows & Projects](../windows-and-projects.md) for more details.

### `--diff <OLD_PATH> <NEW_PATH>`

Open a diff view comparing two files. Can be specified multiple times:

```sh
kurocto --diff file1.txt file2.txt
kurocto --diff old.rs new.rs --diff old2.rs new2.rs
```

### `--foreground`

Run Kurocto in the foreground, keeping the terminal attached. Useful for debugging:

```sh
kurocto --foreground
```

### `--user-data-dir <DIR>`

Use a custom directory for all user data (database, extensions, logs) instead of the default location:

```sh
kurocto --user-data-dir ~/.zed-custom
```

Default locations:

- **macOS:** `~/Library/Application Support/Kurocto`
- **Linux:** `$XDG_DATA_HOME/kurocto` (typically `~/.local/share/kurocto`)
- **Windows:** `%LOCALAPPDATA%\Kurocto`

### `-v`, `--version`

Print Kurocto's version and exit:

```sh
kurocto --version
```

### `--completions <SHELL>`

Generate shell completions for the `kurocto` CLI:

#### Bash

Add to `~/.bashrc`:

```bash
eval "$(kurocto --completions bash)"
```

#### Elvish

Add to `~/.config/elvish/rc.elv`:

```elvish
set edit:completion:arg-completer[kurocto] = { |@args|
    eval (kurocto --completions elvish | slurp)
    $edit:completion:arg-completer[kurocto] $@args
}
```

#### Fish

Add to `~/.config/fish/config.fish`:

```fish
kurocto --completions fish | source
```

#### Nushell

Add to `~/.config/nushell/config.nu`:

```nu
mkdir ($nu.data-dir | path join "vendor/autoload")
^kurocto --completions nushell | save --force ($nu.data-dir | path join "vendor/autoload/kurocto.nu")
```

#### Powershell

Add to `$PROFILE`:

```powershell
(&kurocto --completions powershell) | Out-String | Invoke-Expression
```

#### Zsh

Add to `~/.zshrc`:

```zsh
eval "$(kurocto --completions zsh)"
```

### `--uninstall`

Uninstall Kurocto and remove all related files (macOS and Linux only):

```sh
kurocto --uninstall
```

### `--zed <PATH>`

Specify a custom path to the Kurocto application or binary:

```sh
kurocto --zed /path/to/Kurocto.app myfile.txt
```

## Reading from Standard Input

Read content from stdin by passing `-` as the path:

```sh
echo "Hello, World!" | kurocto -
cat myfile.txt | kurocto -
ps aux | kurocto -
```

This creates a temporary file with the stdin content and opens it in Kurocto.

## URL Handling

The CLI can open `kurocto://`, `file://`, and `ssh://` URLs:

```sh
kurocto kurocto://settings
kurocto file:///Users/whatever/.zshrc
kurocto ssh://me@example.com/abs/path
kurocto ssh://me@example.com:/abs/path
kurocto ssh://me@example.com/~/project
kurocto ssh://me@example.com:~/project
```

## Using Kurocto as Your Default Editor

Set Kurocto as your default editor for Git and other tools:

```sh
export EDITOR="kurocto --wait"
export VISUAL="kurocto --wait"
```

Add these lines to your shell configuration file (e.g., `~/.bashrc`, `~/.zshrc`).

## macOS: Switching Release Channels

On macOS, you can launch a specific release channel by passing the channel name as the first argument:

```sh
kurocto --stable myfile.txt
kurocto --preview myfile.txt
kurocto --nightly myfile.txt
```

## WSL Integration (Windows)

On Windows, the CLI supports opening paths from WSL distributions. This is handled automatically when launching Kurocto from within WSL.

## Exit Codes

| Code | Meaning                           |
| ---- | --------------------------------- |
| `0`  | Success                           |
| `1`  | Error (details printed to stderr) |

When using `--wait`, the exit code reflects whether the files were saved before closing.
