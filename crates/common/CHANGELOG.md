# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-common-v0.2.1...freya-common-v0.3.0) - 2024-12-23

### Added

- `a11y_auto_focus` (#878)
- Optional uncontrolled accessibility IDs (#867)
- Incremental Accessibility Tree (#853)
- `PluginHandle` (#793)
- `winit` v0.30.0 + `glutin-winit` v0.5.0 + `accesskit` v0.14.0 + `accesskit_winit` v0.20.0  (#598)
- Queued focus (#650)
- Revamp internal text selection (#647)
- `WithWindow` event (#626)
- Close app with `use_platform` (#613)
- Add window drag area (#597)

### Fixed

- Skip updated but also removed accessibility nodes (#964)

### Other

- Resolve conflicts
- Simplify the `VirtualDOM` polling (#729)
- `rustfmt.toml` (#689)
- release-plz.toml
- Only release crates under /crates
- Rust 1.78 (#600)
