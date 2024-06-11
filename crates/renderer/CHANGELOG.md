# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-renderer-v0.2.1...freya-renderer-v0.3.0) - 2024-06-11

### Added
- Expose scale factor ([#607](https://github.com/marc2332/freya/pull/607))
- Reactive scale factor ([#606](https://github.com/marc2332/freya/pull/606))
- `winit` v0.30.0 + `glutin-winit` v0.5.0 + `accesskit` v0.14.0 + `accesskit_winit` v0.20.0  ([#598](https://github.com/marc2332/freya/pull/598))
- Tree-like explorer for devtools ([#684](https://github.com/marc2332/freya/pull/684))
- More reliable devtools ([#667](https://github.com/marc2332/freya/pull/667))
- Queued focus ([#650](https://github.com/marc2332/freya/pull/650))
- Revamp internal text selection ([#647](https://github.com/marc2332/freya/pull/647))
- Reactive Window data ([#637](https://github.com/marc2332/freya/pull/637))
- Reactive Platform data ([#635](https://github.com/marc2332/freya/pull/635))
- `use_preferred_theme` ([#631](https://github.com/marc2332/freya/pull/631))
- `WithWindow` event ([#626](https://github.com/marc2332/freya/pull/626))
- Close app with `use_platform` ([#613](https://github.com/marc2332/freya/pull/613))
- Add window drag area ([#597](https://github.com/marc2332/freya/pull/597))

### Fixed
- Proper accessibility reactivity ([#648](https://github.com/marc2332/freya/pull/648))
- Out of sync element ids on events ([#609](https://github.com/marc2332/freya/pull/609))

### Other
- `rustfmt.toml` ([#689](https://github.com/marc2332/freya/pull/689))
- process all queued keyboard events at once ([#629](https://github.com/marc2332/freya/pull/629))
- release-plz.toml
- Only release crates under /crates
- Fix typo on `with_default_font`
