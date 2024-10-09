# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-native-core-v0.2.1...freya-native-core-v0.3.0) - 2024-10-09

### Added

- add attributes for most AccessKit properties ([#882](https://github.com/marc2332/freya/pull/882))
- multiple borders ([#889](https://github.com/marc2332/freya/pull/889))
- Rename `pointerover` event to `pointermove` ([#897](https://github.com/marc2332/freya/pull/897))
- Only focus focusable nodes ([#884](https://github.com/marc2332/freya/pull/884))
- `a11y_auto_focus` ([#878](https://github.com/marc2332/freya/pull/878))
- Focus-based keyboard events ([#877](https://github.com/marc2332/freya/pull/877))
- Rename a11y attributes ([#869](https://github.com/marc2332/freya/pull/869))
- Rename `mouseover` to `mousemove` ([#865](https://github.com/marc2332/freya/pull/865))
- Testing events utils ([#864](https://github.com/marc2332/freya/pull/864))
- Improved nodes events states ([#859](https://github.com/marc2332/freya/pull/859))
- `spacing` attribute ([#834](https://github.com/marc2332/freya/pull/834))
- `highlight_mode` attribute ([#704](https://github.com/marc2332/freya/pull/704))

### Fixed

- Use collateral event to check if event is allowed ([#890](https://github.com/marc2332/freya/pull/890))
- Dont trigger mouse enter on touch move ([#888](https://github.com/marc2332/freya/pull/888))
- *(deps)* update rust crate dashmap to v6 ([#739](https://github.com/marc2332/freya/pull/739))
- *(deps)* update rust crate dashmap to v6 ([#726](https://github.com/marc2332/freya/pull/726))

### Other

- Revert "fix(deps): update rust crate dashmap to v6 ([#726](https://github.com/marc2332/freya/pull/726))" ([#730](https://github.com/marc2332/freya/pull/730))
- `rustfmt.toml` ([#689](https://github.com/marc2332/freya/pull/689))
- Run clippy in tests and examples
