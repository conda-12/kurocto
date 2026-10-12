# Kurocto
Kurocto is a code editor. It is a modified version of [Zed](https://github.com/zed-industries/zed), which is developed by Zed Industries, Inc.
Kurocto is an independent project. It is not an official Zed release, and it does not use Zed accounts or Zed-operated services.

---

### Relationship to Zed

Kurocto was forked from Zed at upstream commit `93d7456688` (dated 2026-10-10) and has been modified since. The changes are recorded in this repository's git history. Among other things, Kurocto removes Zed account sign-in, Zed-hosted AI services, collaboration, telemetry and crash uploads, and automatic updates from Zed's servers.

### Developing Kurocto

- [Building for macOS](./docs/src/development/macos.md)
- [Building for Linux](./docs/src/development/linux.md)
- [Building for Windows](./docs/src/development/windows.md)

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

### Licensing

Kurocto's source code is licensed primarily under GPL-3.0-or-later, with Apache-2.0 components where marked, the same as the Zed source code it is based on. See [LICENSE-GPL](./LICENSE-GPL) and [LICENSE-APACHE](./LICENSE-APACHE). Copyright notices from Zed Industries, Inc. and other contributors are retained.

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/zed-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/zed-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).
