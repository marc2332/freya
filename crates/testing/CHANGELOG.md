# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2](https://github.com/marc2332/freya/compare/freya-testing-v0.2.1...freya-testing-v0.2.2) - 2024-09-28

### Added

- Focus-based keyboard events ([#877](https://github.com/marc2332/freya/pull/877))
- Optional uncontrolled accessibility IDs ([#867](https://github.com/marc2332/freya/pull/867))
- Rename `mouseover` to `mousemove` ([#865](https://github.com/marc2332/freya/pull/865))
- Testing events utils ([#864](https://github.com/marc2332/freya/pull/864))
- Improved nodes events states ([#859](https://github.com/marc2332/freya/pull/859))
- Incremental Accessibility Tree ([#853](https://github.com/marc2332/freya/pull/853))
- `PluginHandle` ([#793](https://github.com/marc2332/freya/pull/793))
- Canvas snapshots for `freya-testing` ([#720](https://github.com/marc2332/freya/pull/720))
- Use System fonts ([#661](https://github.com/marc2332/freya/pull/661))
- Expose scale factor ([#607](https://github.com/marc2332/freya/pull/607))
- Revamp internal text selection ([#647](https://github.com/marc2332/freya/pull/647))
- Reactive Window data ([#637](https://github.com/marc2332/freya/pull/637))
- Reactive Platform data ([#635](https://github.com/marc2332/freya/pull/635))
- `use_preferred_theme` ([#631](https://github.com/marc2332/freya/pull/631))
- Improved special text editing support ([#622](https://github.com/marc2332/freya/pull/622))

### Fixed

- Prevent opacity from clipping the node bounds ([#764](https://github.com/marc2332/freya/pull/764))
- Consider corner radius for events and overflow clipping ([#768](https://github.com/marc2332/freya/pull/768))
- Out of sync element ids on events ([#609](https://github.com/marc2332/freya/pull/609))

### Other

- Resolve conflicts
- Rethink mutations writer ([#731](https://github.com/marc2332/freya/pull/731))
- Simplify the `VirtualDOM` polling ([#729](https://github.com/marc2332/freya/pull/729))
- Rename node states to follow an unified naming ([#713](https://github.com/marc2332/freya/pull/713))
- `rustfmt.toml` ([#689](https://github.com/marc2332/freya/pull/689))
- process all queued keyboard events at once ([#629](https://github.com/marc2332/freya/pull/629))
- release-plz.toml
- Only release crates under /crates
- Clean up debris