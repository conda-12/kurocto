---
title: Extensions
description: "Extend Kurocto with themes, language support, AI tools, and more through the extension system."
---

# Extensions

Kurocto lets you add new functionality using user-defined extensions.

- [Installing Extensions](./extensions/installing-extensions.md)
- [Extension Capabilities](./extensions/capabilities.md)
- [Developing Extensions](./extensions/developing-extensions.md)
  - [Developing Debugger Extensions](./extensions/debugger-extensions.md)
  - [Developing Icon Themes](./extensions/icon-themes.md)
  - [Developing Language Extensions](./extensions/languages.md)
  - [Developing MCP Servers](./extensions/mcp-extensions.md)
  - [Developing Snippets](./extensions/snippets.md)
  - [Developing Themes](./extensions/themes.md)

# Publishing your extension

Kurocto does not run an extension registry, so there is nowhere to publish an extension to yet. Extensions can be shared as source and installed from a local directory as [dev extensions](./extensions/developing-extensions.md#developing-an-extension-locally), or served from a compatible registry configured with [`extension_registry_url`](./extensions/installing-extensions.md).
