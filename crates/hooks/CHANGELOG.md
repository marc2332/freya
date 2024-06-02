# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-hooks-v0.2.1...freya-hooks-v0.3.0) - 2024-06-02

### Added
- Select all text ([#652](https://github.com/marc2332/freya/pull/652))
- Revamp internal text selection ([#647](https://github.com/marc2332/freya/pull/647))
- Reactive Platform data ([#635](https://github.com/marc2332/freya/pull/635))
- `use_preferred_theme` ([#631](https://github.com/marc2332/freya/pull/631))
- Remove text with Delete ([#644](https://github.com/marc2332/freya/pull/644))
- Text dragging with shift and cursor ([#642](https://github.com/marc2332/freya/pull/642))
- Delete text selection with backspace ([#640](https://github.com/marc2332/freya/pull/640))
- Improved special text editing support ([#622](https://github.com/marc2332/freya/pull/622))
- `WithWindow` event ([#626](https://github.com/marc2332/freya/pull/626))
- `placeholder` for Input ([#616](https://github.com/marc2332/freya/pull/616))
- Close app with `use_platform` ([#613](https://github.com/marc2332/freya/pull/613))
- Add window drag area ([#597](https://github.com/marc2332/freya/pull/597))

### Fixed
- Proper accessibility reactivity ([#648](https://github.com/marc2332/freya/pull/648))
- Fix performance dropping rapidly after selecting a text for some time ([#624](https://github.com/marc2332/freya/pull/624))

### Other
- Use single position cursor ([#653](https://github.com/marc2332/freya/pull/653))
- Add tests for shift + click text selection
- Merge branch 'main' of https://github.com/marc2332/freya
- Fix shift+click selection in virtualized editors
- Enable a `use_theme` doc test
- Fix text selection in some cases
- release-plz.toml
- Only release crates under /crates
