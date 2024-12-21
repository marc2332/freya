# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-devtools-v0.2.1...freya-devtools-v0.3.0) - 2024-12-21

### Added

- multiple borders (#889)
- Ergonomic improvements in ScrollView (#858)
- Add missing gradient functions (#776)
- Small UI improvements in the devtools
- Tree-like explorer for devtools (#684)
- Tab and Tabsbar components (#673)
- More reliable devtools (#667)
- Persisted devtools routing (#657)
- `use_preferred_theme` (#631)

### Fixed

- Use `use_applied_theme` in devtools
- Use the real text height for layout (#932)
- Do a fullrerender when a devtools node is selected
- Small devtool fixes
- Show missing attributes in devtools (#801)

### Other

- add support for `fill` in `svg` (#797)
- Some devtools UI adjustements
- Clean up Tabs components
- `rustfmt.toml` (#689)
- release-plz.toml
- Only release crates under /crates
