---
title: Install Kurocto - macOS, Linux, Windows
description: Kurocto is currently built from source on macOS, Linux, and Windows.
---

# Installing Kurocto

Kurocto does not have published builds, a download site, package manager packages, or an install script yet. It is currently built from source:

- [Building for macOS](./development/macos.md)
- [Building for Linux](./development/linux.md)
- [Building for Windows](./development/windows.md)

## System Requirements

### macOS

Kurocto supports the following macOS releases:

| Version       | Codename | Apple Status   | Kurocto Status          |
| ------------- | -------- | -------------- | ------------------- |
| macOS 26.x    | Tahoe    | Supported      | Supported           |
| macOS 15.x    | Sequoia  | Supported      | Supported           |
| macOS 14.x    | Sonoma   | Supported      | Supported           |
| macOS 13.x    | Ventura  | Supported      | Supported           |
| macOS 12.x    | Monterey | EOL 2024-09-16 | Supported           |
| macOS 11.x    | Big Sur  | EOL 2023-09-26 | Partially Supported |
| macOS 10.15.x | Catalina | EOL 2022-09-12 | Partially Supported |

#### Mac Hardware

Kurocto supports machines with Intel (x86_64) or Apple (aarch64) processors that meet the above macOS requirements:

- MacBook Pro (Early 2015 and newer)
- MacBook Air (Early 2015 and newer)
- MacBook (Early 2016 and newer)
- Mac Mini (Late 2014 and newer)
- Mac Pro (Late 2013 or newer)
- iMac (Late 2015 and newer)
- iMac Pro (all models)
- Mac Studio (all models)

### Linux

Kurocto supports 64-bit Intel/AMD (x86_64) and 64-bit Arm (aarch64) processors.

Kurocto requires a Vulkan 1.3 driver and the following desktop portals:

- `org.freedesktop.portal.FileChooser`
- `org.freedesktop.portal.OpenURI`
- `org.freedesktop.portal.Secret` or `org.freedesktop.Secrets`

### Windows

Kurocto supports the following Windows releases:
| Version | Kurocto Status |
| ------------------------- | ------------------- |
| Windows 11, version 22H2 and later | Supported |
| Windows 10, version 1903 and later | Supported |

A 64-bit operating system is required to run Kurocto.

#### Windows Hardware

Kurocto supports machines with x64 (Intel, AMD) or Arm64 (Qualcomm) processors that meet the following requirements:

- Graphics: A GPU that supports DirectX 11 (most PCs from 2012+).
- Driver: Current NVIDIA/AMD/Intel/Qualcomm driver (not the Microsoft Basic Display Adapter).

### FreeBSD

Can be built [from source](./development/freebsd.md).

### Web

Not supported at this time. See the upstream [Platform Support issue](https://github.com/zed-industries/zed/issues/5391).
