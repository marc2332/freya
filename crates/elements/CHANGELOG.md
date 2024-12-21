# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-elements-v0.2.0...freya-elements-v0.3.0) - 2024-12-21

### Added

- New `OverflowContent` component (#1011)
- New `AnimatedPosition` component (#1013)
- flex support (#920)
- `text_height` attribute to control the height behavior of text (#976)
- add attributes for most AccessKit properties (#882)
- multiple borders (#889)
- per-side border widths (#836)
- Rename `pointerover` event to `pointermove` (#897)
- Only focus focusable nodes (#884)
- `a11y_auto_focus` (#878)
- Focus-based keyboard events (#877)
- Rename a11y attributes (#869)
- Rename `mouseover` to `mousemove` (#865)
- Improved nodes events states (#859)
- `spacing` attribute (#834)
- Support `space-between`/`space-around`/`space-evenly` alignments (#758)
- Use System fonts (#661)
- `highlight_mode` attribute (#704)
- Built-in vertical alignment for text (#701)
- `onpress` event for `Button` (#601)

### Fixed

- Small typos

### Other

- add support for `fill` in `svg` (#797)
- Add missing 0.2 docs (#916)
- organize inconsistent attributes (#950)
- Update definitions.rs
- Update main_align_cross_align.md
- Rename some events files
- Documents editor example (#846)
- Fix color syntax link in background.md
- `rustfmt.toml` (#689)
- release-plz.toml
- Only release crates under /crates
