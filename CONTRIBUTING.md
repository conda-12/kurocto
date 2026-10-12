# Contributing to Kurocto

Kurocto is a modified version of [Zed](https://github.com/zed-industries/zed). Zed's contribution process, issue tracker, community programs, Code of Conduct and Contributor License Agreement belong to Zed Industries and do not apply here. Please do not send Kurocto issues or pull requests to the Zed repository.

Kurocto's own contribution policy is still being written. Until it is, the guidance below covers the practical side of sending a change.

## Sending changes

**Build and run Kurocto locally, then manually test your changes before opening a PR.** Follow the guide for [macOS](./docs/src/development/macos.md), [Linux](./docs/src/development/linux.md), [Windows](./docs/src/development/windows.md), or [FreeBSD](./docs/src/development/freebsd.md).

When preparing your PR:

- **Review your own work**, including any AI-assisted code. Follow the [pull request template](./.github/pull_request_template.md?plain=1) for testing details and the self-review checklist.
- **For visual changes, attach screenshots or a video** and work through the [UI/UX checklist](./docs/src/development/ui-checklist.md). For non-visual improvements and changes, include benchmarks or other artifacts produced when testing.
- **Keep each PR about one thing**, leaving unrelated features and refactoring for another PR.
- **Include tests** for non-trivial changes.

## Licensing

Kurocto's source code is licensed primarily under GPL-3.0-or-later, with Apache-2.0 components where marked. See the [README](./README.md#licensing) for how third-party license information is checked.
