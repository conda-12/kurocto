---
title: Uninstall
description: "This guide covers how to uninstall Kurocto on different operating systems."
---

# Uninstall

This guide covers how to uninstall Kurocto on different operating systems.

## macOS

1. Quit Kurocto if it's running
2. Open Finder and go to the folder that contains Kurocto (usually Applications)
3. Drag Kurocto to the Trash (or right-click and select "Move to Trash")
4. Empty the Trash

### Removing User Data (Optional)

To completely remove all Kurocto configuration files and data:

1. Open Finder
2. Press `Cmd + Shift + G` to open "Go to Folder"
3. Delete the following directories if they exist:
   - `~/Library/Application Support/Kurocto`
   - `~/Library/Saved Application State/dev.kurocto.Kurocto.savedState`
   - `~/Library/Logs/Kurocto`
   - `~/Library/Caches/dev.kurocto.Kurocto`
   - `~/Library/Caches/Kurocto`
   - `~/.config/kurocto`
   - `~/.local/state/Kurocto`

## Linux

### Standard Uninstall

If Kurocto was installed with `script/install-linux`, run:

```sh
kurocto --uninstall
```

You'll be prompted whether to keep or delete your preferences. After making a choice, you should see a message that Kurocto was successfully uninstalled.

If the `kurocto` command is not found in your PATH, try:

```sh
$HOME/.local/bin/kurocto --uninstall
```

or:

```sh
$HOME/.local/kurocto.app/bin/kurocto --uninstall
```

### Package Manager

If you installed Kurocto using a package manager (such as Flatpak, Snap, or a distribution-specific package manager), consult that package manager's documentation for uninstallation instructions.

### Manual Removal

If the uninstall command fails or Kurocto was installed to a custom location, you can manually remove:

- Installation directory: `~/.local/kurocto.app` (or your custom installation path)
- Binary symlink: `~/.local/bin/kurocto`
- Configuration and data: `~/.config/kurocto`

## Windows

### Standard Installation

1. Quit Kurocto if it's running
2. Open Settings (Windows key + I)
3. Go to "Apps" > "Installed apps" (or "Apps & features" on Windows 10)
4. Search for "Kurocto"
5. Click the three dots menu next to Kurocto and select "Uninstall"
6. Follow the prompts to complete the uninstallation

Alternatively, you can:

1. Open the Start menu
2. Right-click on Kurocto
3. Select "Uninstall"

### Removing User Data (Optional)

To completely remove all Kurocto configuration files and data:

1. Press `Windows key + R` to open Run
2. Type `%APPDATA%` and press Enter
3. Delete the `Kurocto` folder if it exists
4. Press `Windows key + R` again, type `%LOCALAPPDATA%` and press Enter
5. Delete the `Kurocto` folder if it exists

## Troubleshooting

If you encounter issues during uninstallation:

- **macOS/Windows**: Ensure Kurocto is completely quit before attempting to uninstall. Check Activity Manager (macOS) or Task Manager (Windows) for any running Kurocto processes.
- **Linux**: If the uninstall script fails, check the error message and consider manual removal of the directories listed above.
- **All platforms**: If you want to start fresh while keeping Kurocto installed, you can delete the configuration directories instead of uninstalling the application entirely.

For additional help, see our [Linux-specific documentation](./linux.md).
