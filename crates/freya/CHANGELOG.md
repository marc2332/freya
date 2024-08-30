# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.3](https://github.com/marc2332/freya/compare/freya-v0.2.2...freya-v0.2.3) - 2024-08-30

### Added
- Add `performance-overlay` feature to `freya` crate ([#809](https://github.com/marc2332/freya/pull/809))
- Allow custom Tokio Runtimes ([#765](https://github.com/marc2332/freya/pull/765))
- `winit` v0.30.0 + `glutin-winit` v0.5.0 + `accesskit` v0.14.0 + `accesskit_winit` v0.20.0  ([#598](https://github.com/marc2332/freya/pull/598))
- More reliable devtools ([#667](https://github.com/marc2332/freya/pull/667))
- Do not re-export freya-testing ([#669](https://github.com/marc2332/freya/pull/669))
- `onpress` event for `Button` ([#601](https://github.com/marc2332/freya/pull/601))
- `use_preferred_theme` ([#631](https://github.com/marc2332/freya/pull/631))

### Fixed
- Properly launch the tokio runtime
- Require to pass both the width and height for window size at once when desired ([#757](https://github.com/marc2332/freya/pull/757))

### Other
- Repair the color value in the comment information ([#694](https://github.com/marc2332/freya/pull/694))
- `rustfmt.toml` ([#689](https://github.com/marc2332/freya/pull/689))
- release-plz.toml
- Only release crates under /crates
