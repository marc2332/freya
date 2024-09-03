# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-core-v0.2.1...freya-core-v0.3.0) - 2024-09-03

### Added
- `spacing` attribute ([#834](https://github.com/marc2332/freya/pull/834))
- `PluginHandle` ([#793](https://github.com/marc2332/freya/pull/793))
- Ignore unnecessary dioxus vdom mutations ([#821](https://github.com/marc2332/freya/pull/821))
- Avoid copying images when rendering ([#808](https://github.com/marc2332/freya/pull/808))
- Add missing gradient functions ([#776](https://github.com/marc2332/freya/pull/776))
- Support `space-between`/`space-around`/`space-evenly` alignments ([#758](https://github.com/marc2332/freya/pull/758))
- Use System fonts ([#661](https://github.com/marc2332/freya/pull/661))
- `highlight_mode` attribute ([#704](https://github.com/marc2332/freya/pull/704))
- Built-in vertical alignment for text ([#701](https://github.com/marc2332/freya/pull/701))
- Expose scale factor ([#607](https://github.com/marc2332/freya/pull/607))
- Reactive scale factor ([#606](https://github.com/marc2332/freya/pull/606))
- `winit` v0.30.0 + `glutin-winit` v0.5.0 + `accesskit` v0.14.0 + `accesskit_winit` v0.20.0  ([#598](https://github.com/marc2332/freya/pull/598))
- Revamp internal text selection ([#647](https://github.com/marc2332/freya/pull/647))
- Reactive Window data ([#637](https://github.com/marc2332/freya/pull/637))
- Reactive Platform data ([#635](https://github.com/marc2332/freya/pull/635))
- `use_preferred_theme` ([#631](https://github.com/marc2332/freya/pull/631))

### Fixed
- Show missing attributes in devtools ([#801](https://github.com/marc2332/freya/pull/801))
- Prevent opacity from clipping the node bounds ([#764](https://github.com/marc2332/freya/pull/764))
- Consider corner radius for events and overflow clipping ([#768](https://github.com/marc2332/freya/pull/768))
- Fix `unfocus` of accesibility nodes ([#755](https://github.com/marc2332/freya/pull/755))
- Skip DOM Nodes loaded in the same mutations run ([#744](https://github.com/marc2332/freya/pull/744))
- Invalidate layout of modified text nodes
- *(deps)* update all non-major dependencies ([#578](https://github.com/marc2332/freya/pull/578))
- Proper accessibility reactivity ([#648](https://github.com/marc2332/freya/pull/648))
- Fix performance dropping rapidly after selecting a text for some time ([#624](https://github.com/marc2332/freya/pull/624))
- Out of sync element ids on events ([#609](https://github.com/marc2332/freya/pull/609))

### Other
- Add opengl_rtt example. ([#813](https://github.com/marc2332/freya/pull/813))
- *(deps)* update all non-major dependencies ([#749](https://github.com/marc2332/freya/pull/749))
- Revert part of [#731](https://github.com/marc2332/freya/pull/731)
- Rethink mutations writer ([#731](https://github.com/marc2332/freya/pull/731))
- Reorganize `freya-renderer` ([#715](https://github.com/marc2332/freya/pull/715))
- Rename node states to follow an unified naming ([#713](https://github.com/marc2332/freya/pull/713))
- Move rendering to `freya-core` ([#712](https://github.com/marc2332/freya/pull/712))
- `rustfmt.toml` ([#689](https://github.com/marc2332/freya/pull/689))
- Run clippy in tests and examples
- process all queued keyboard events at once ([#629](https://github.com/marc2332/freya/pull/629))
- release-plz.toml
- Only release crates under /crates
