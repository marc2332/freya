# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-node-state-v0.2.1...freya-node-state-v0.3.0) - 2024-12-24

### Added

- New `AnimatedPosition` component (#1013)
- flex support (#920)
- `text_height` attribute to control the height behavior of text (#976)
- Parent children based accessibility relations (#958)
- add attributes for most AccessKit properties (#882)
- multiple borders (#889)
- per-side border widths (#836)
- Support RootPercentage in `calc()` (#907)
- Only focus focusable nodes (#884)
- `a11y_auto_focus` (#878)
- Rename a11y attributes (#869)
- Optional uncontrolled accessibility IDs (#867)
- Incremental Accessibility Tree (#853)
- `spacing` attribute (#834)
- Support percentage of auto in layout (#784)
- Add missing gradient functions (#776)
- Panic when an attribute has a wrongly-formatted value, but only in debug builds to easily spot bugs (#759)
- Support `space-between`/`space-around`/`space-evenly` alignments (#758)
- `highlight_mode` attribute (#704)
- Expose scale factor (#607)
- Reactive scale factor (#606)
- Revamp internal text selection (#647)

### Fixed

- Avoid trigering side effects in orphan nodes updates (#959)
- Fix radial gradient
- Use the real text height for layout (#932)
- linear gradient angles (#921)
- Use individual methods to set decoration (#842)
- Support `none` for background colors

### Other

- Add prefixes and parentheses to calc function ([#988](https://github.com/marc2332/freya/pull/988))
- add support for `fill` in `svg` (#797)
- Fmt and fix clippy warnings from 1.82
- Split core render functions (#937)
- Merge branch 'main' into feat/incremental-rendering
- Add opengl_rtt example. ([#813](https://github.com/marc2332/freya/pull/813))
- Allow `none` for non-text colors attributes
- Compile error for attribute parsing in --release
- Rename node states to follow an unified naming (#713)
- Move rendering to `freya-core` (#712)
- Rust 1.79 (#710)
- `rustfmt.toml` (#689)
- Run clippy in tests and examples
- release-plz.toml
- Only release crates under /crates
