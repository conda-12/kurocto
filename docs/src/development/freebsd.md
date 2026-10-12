---
title: Building Kurocto for FreeBSD
description: "Guide to building Kurocto for freebsd for Kurocto development."
---

# Building Kurocto for FreeBSD

FreeBSD is not currently a supported platform, so this guide is a work in progress.

## Repository

Clone the [Kurocto repository](https://github.com/conda-12/kurocto).

## Dependencies

- Install the necessary system packages and rustup:

  ```sh
  script/freebsd
  ```

  If preferred, you can inspect [`script/freebsd`](https://github.com/conda-12/kurocto/blob/main/script/freebsd) and perform the steps manually.

## Building from source

Once the dependencies are installed, you can build Kurocto using [Cargo](https://doc.rust-lang.org/cargo/).

For a debug build of the editor:

```sh
cargo run
```

And to run the tests:

```sh
cargo test --workspace
```

In release mode, the primary user interface is the `cli` crate. You can run it in development with:

```sh
cargo run -p cli
```

See [Issue #15309: FreeBSD Support] and [Discussion #29550: Unofficial FreeBSD port for Zed] for more.

## Troubleshooting

### Cargo errors claiming that a dependency is using unstable features

Try `cargo clean` and `cargo build`.
