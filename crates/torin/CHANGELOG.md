# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/torin-v0.2.0...torin-v0.3.0) - 2024-12-24

### Added

- New `AnimatedPosition` component (#1013)
- flex support (#920)
- Support RootPercentage in `calc()` (#907)
- Calc operator precedence (#838)
- `spacing` attribute (#834)
- Refactor some parts of Torin (#807)
- Support percentage of auto in layout (#784)
- Support `space-between`/`space-around`/`space-evenly` alignments (#758)
- Reactive scale factor (#606)

### Fixed

- *(torin)* Also adjust post-layout area in double phase measurements (#1022)
- Handle reordedering of keyed children (#1015)
- Layout references not triggering (#934)
- Properly adjust accumulated sizes when using padding (#894)
- Proper incremental redraws for elements with outer or center borders (#879)
- *(torin)* Ignore absolute nodes for spacing and alignments (#873)
- Fix release-plz CI by renaming the torin readme file in Cargo.toml
- Small typos
- Show missing attributes in devtools (#801)

### Other

- Revert "Fix scale factor behavior in `calc()`" ([#1008](https://github.com/marc2332/freya/pull/1008))
- Fix scale factor behavior in `calc()` ([#1006](https://github.com/marc2332/freya/pull/1006))
- Add prefixes and parentheses to calc function ([#988](https://github.com/marc2332/freya/pull/988))
- More formatting
- Fmt and fix clippy warnings from 1.82
- Avoid marking as dirty fixed-size nodes with non-start alignments or layout references if they haven't actually been modified (#919)
- Be more generous for rotated dirty areas
- `rustfmt.toml` (#689)
- Run clippy in tests and examples
- release-plz.toml
- Only release crates under /crates
