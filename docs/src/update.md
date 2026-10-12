---
title: Update Zed
description: "Zed does not update itself. New versions are installed manually."
---

# Update Zed

Zed does not update itself. It does not check for, download, or install new versions, and there is no setting that turns this on.

## How to update

To update, install a newer version manually, the same way you installed the current one. If you built Zed from source, fetch the latest changes and rebuild.

Selecting **Check for Updates** from the application menu (or running {#action zed::CheckForUpdates} from the Command Palette) shows a reminder that updates are installed manually. It does not contact any server.

## How to check your current version

To check which version of Zed you're using:

Open the Command Palette (Cmd+Shift+P on macOS, Ctrl+Shift+P on Linux/Windows).

Type and select {#action zed::About}. A modal will appear with your version information.
