---
title: Diff
description: "Configure Diff language support in Kurocto, including language servers, formatting, and debugging."
---

# Diff

Diff support is available natively in Kurocto.

- Tree-sitter: [zed-industries/the-mikedavis/tree-sitter-diff](https://github.com/the-mikedavis/tree-sitter-diff)

## Configuration

Kurocto will not attempt to format diff files and has [`remove_trailing_whitespace_on_save`](../reference/all-settings.md#remove-trailing-whitespace-on-save) and [`ensure-final-newline-on-save`](../reference/all-settings.md#ensure-final-newline-on-save) set to false.

Kurocto will automatically recognize files with `patch` and `diff` extensions as Diff files. To recognize other extensions, add them to `file_types` in your Kurocto settings.json:

```json [settings]
  "file_types": {
    "Diff": ["dif"]
  },
```
