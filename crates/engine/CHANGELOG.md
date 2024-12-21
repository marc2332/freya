# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-engine-v0.2.2...freya-engine-v0.3.0) - 2024-12-21

### Added

- Force SVG root element to have the specified size (#850)
- `text_height` attribute to control the height behavior of text (#976)
- per-side border widths (#836)
- infer accesskit properties from node state (#855)
- Add missing gradient functions (#776)
- Canvas snapshots for `freya-testing` (#720)
- Skia-safe v0.75 (#716)
- Improved special text editing support (#622)

### Fixed

- Use individual methods to set decoration (#842)
- Prevent opacity from clipping the node bounds (#764)
- Add missing skia mocked methods

### Other

- add support for `fill` in `svg` (#797)
- Merge branch 'main' into feat/incremental-rendering
- `rustfmt.toml` (#689)
- release-plz.toml
- Only release crates under /crates
