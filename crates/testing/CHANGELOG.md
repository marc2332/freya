# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-testing-v0.2.1...freya-testing-v0.3.0) - 2024-12-23

### Added

- Allow passing state context in freya-testing (#981)
- in-memory canvas snapshots (#979)
- Focus-based keyboard events (#877)
- Optional uncontrolled accessibility IDs (#867)
- Rename `mouseover` to `mousemove` (#865)
- Testing events utils (#864)
- Improved nodes events states (#859)
- Incremental Accessibility Tree (#853)
- `PluginHandle` (#793)
- Canvas snapshots for `freya-testing` (#720)
- Use System fonts (#661)
- Expose scale factor (#607)
- Revamp internal text selection (#647)
- Reactive Window data (#637)
- Reactive Platform data (#635)
- `use_preferred_theme` (#631)
- Improved special text editing support (#622)

### Fixed

- Use the config size for the compositor dirty area in freya-testing when resizing (#989)
- Prevent opacity from clipping the node bounds (#764)
- Consider corner radius for events and overflow clipping (#768)
- Out of sync element ids on events (#609)

### Other

- Resolve conflicts
- Rethink mutations writer (#731)
- Simplify the `VirtualDOM` polling (#729)
- Rename node states to follow an unified naming (#713)
- `rustfmt.toml` (#689)
- process all queued keyboard events at once (#629)
- release-plz.toml
- Only release crates under /crates
- Clean up debris
