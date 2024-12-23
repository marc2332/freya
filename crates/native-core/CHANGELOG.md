# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-native-core-v0.2.1...freya-native-core-v0.3.0) - 2024-12-23

### Added

- Force SVG root element to have the specified size (#850)
- `text_height` attribute to control the height behavior of text (#976)
- add attributes for most AccessKit properties (#882)
- multiple borders (#889)
- Rename `pointerover` event to `pointermove` (#897)
- Only focus focusable nodes (#884)
- `a11y_auto_focus` (#878)
- Focus-based keyboard events (#877)
- Rename a11y attributes (#869)
- Rename `mouseover` to `mousemove` (#865)
- Testing events utils (#864)
- Improved nodes events states (#859)
- `spacing` attribute (#834)
- `highlight_mode` attribute (#704)

### Fixed

- Handle reordedering of keyed children (#1015)
- Use collateral event to check if event is allowed (#890)
- Dont trigger mouse enter on touch move (#888)
- *(deps)* update rust crate dashmap to v6 (#739)
- *(deps)* update rust crate dashmap to v6 (#726)

### Other

- add support for `fill` in `svg` (#797)
- Remove `anymap` (#1001)
- Remove `DirtyNodesResults` from `native-core` (#999)
- Revert "fix(deps): update rust crate dashmap to v6 ([#726](https://github.com/marc2332/freya/pull/726))" ([#730](https://github.com/marc2332/freya/pull/730))
- `rustfmt.toml` (#689)
- Run clippy in tests and examples
