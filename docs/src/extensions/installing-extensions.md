---
title: Installing Extensions
description: "Install and manage extensions from a local directory or from an extension registry you configure."
---

# Installing Extensions {#installing-extensions}

Extensions add functionality to Zed, including languages, themes, and AI tools.

Open the Extensions page with {#kb zed::Extensions}, or select "Zed > Extensions" from the menu bar. It lists the extensions you have installed.

## Extension Registry {#extension-registry}

No extension registry is configured by default. Without one, nothing is contacted: extensions are not browsed, installed, auto-installed, updated or suggested over the network.

Extensions can always be installed from a local directory. On the Extensions page, click `Install Dev Extension` (or run the {#action zed::InstallDevExtension} action) and select the directory containing the extension. See [Developing Extensions](./developing-extensions.md#developing-an-extension-locally) for the requirements.

To browse, install and update extensions from a registry, set [`extension_registry_url`](../reference/all-settings.md#extension-registry-url) to the base URL of a registry that implements the extension registry API:

```json [settings]
{
  "extension_registry_url": "https://registry.example.com"
}
```

Extension listings and downloads are then requested from that URL, and the IDs of your installed extensions are sent to it when checking for updates. Using a registry operated by someone else, including Zed's, is subject to that operator's terms.

## Installation Location

- On macOS, extensions are installed in `~/Library/Application Support/Zed/extensions`.
- On Linux, they are installed in either `$XDG_DATA_HOME/zed/extensions` or `~/.local/share/zed/extensions`.
- On Windows, the directory is `%LOCALAPPDATA%\Zed\extensions`.

This directory contains two subdirectories:

- `installed`, which contains the source code for each extension.
- `work` which contains files created by the extension itself, such as downloaded language servers.

## Auto-installing

To automate extension installation/uninstallation see the docs for [auto_install_extensions](../reference/all-settings.md#auto-install-extensions). Auto-installation requires an [extension registry](#extension-registry) to be configured; nothing is auto-installed by default.
