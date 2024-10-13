# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/torin-v0.2.0...torin-v0.3.0) - 2024-10-13

### Added

- Support RootPercentage in `calc()` ([#907](https://github.com/marc2332/freya/pull/907))
- Calc operator precedence ([#838](https://github.com/marc2332/freya/pull/838))
- `spacing` attribute ([#834](https://github.com/marc2332/freya/pull/834))
- Refactor some parts of Torin ([#807](https://github.com/marc2332/freya/pull/807))
- Support percentage of auto in layout ([#784](https://github.com/marc2332/freya/pull/784))
- Support `space-between`/`space-around`/`space-evenly` alignments ([#758](https://github.com/marc2332/freya/pull/758))
- Reactive scale factor ([#606](https://github.com/marc2332/freya/pull/606))

### Fixed

- Layout references not triggering ([#934](https://github.com/marc2332/freya/pull/934))
- Properly adjust accumulated sizes when using padding ([#894](https://github.com/marc2332/freya/pull/894))
- Proper incremental redraws for elements with outer or center borders ([#879](https://github.com/marc2332/freya/pull/879))
- *(torin)* Ignore absolute nodes for spacing and alignments ([#873](https://github.com/marc2332/freya/pull/873))
- Fix release-plz CI by renaming the torin readme file in Cargo.toml
- Small typos
- Show missing attributes in devtools ([#801](https://github.com/marc2332/freya/pull/801))

### Other

- Avoid marking as dirty fixed-size nodes with non-start alignments or layout references if they haven't actually been modified ([#919](https://github.com/marc2332/freya/pull/919))
- Be more generous for rotated dirty areas
- `rustfmt.toml` ([#689](https://github.com/marc2332/freya/pull/689))
- Run clippy in tests and examples
- release-plz.toml
- Only release crates under /crates
