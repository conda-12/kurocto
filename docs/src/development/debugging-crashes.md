---
title: Debugging Crashes
description: "Guide to debugging crashes for Zed development."
---

# Debugging Crashes

When Zed panics, it writes the panic message and a backtrace to its log file.

Zed can also create a [minidump](https://chromium.googlesource.com/breakpad/breakpad/+/master/docs/getting_started_with_breakpad.md#the-minidump-file-format) when it panics or crashes. This is off by default: start Zed (or the remote server) with the `ZED_GENERATE_MINIDUMPS=1` environment variable to enable it. Zed then sends a message to a sidecar process that inspects the editor's memory and writes the minidump to `~/Library/Logs/Zed` or `$XDG_DATA_HOME/zed/logs`. You can use this minidump to generate backtraces for all thread stacks.

Minidumps are never uploaded and are not deleted automatically.

Minidumps include useful data, but they are hard to read without spans or symbol information. You can analyze them locally with the source and an unstripped binary (or separate symbols file) for your Zed build, then running:

```sh
zstd -d ~/.local/share/zed/<uuid>.dmp -o minidump.dmp
minidump-stackwalk minidump.dmp
```

Alongside the minidump in your logs directory, you should also see a `<uuid>.json` file with metadata such as the panic message, span, and system specs.

## Using a Debugger

If you can reproduce the crash consistently, use a debugger to inspect program state at the crash point.

For setup details, see [Using a debugger](./debuggers.md#debugging-panics-and-crashes).
