# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-core-v0.2.1...freya-core-v0.3.0) - 2024-12-23

### Added

- Prepare New Docs (#788)
- New `AnimatedPosition` component (#1013)
- Force SVG root element to have the specified size (#850)
- Consider antialising for incremental rendering (#985)
- Allow passing state context in freya-testing (#981)
- `text_height` attribute to control the height behavior of text (#976)
- Parent children based accessibility relations (#958)
- add attributes for most AccessKit properties (#882)
- multiple borders (#889)
- per-side border widths (#836)
- Deterministic order of rendering (#923)
- `TooltipContainer` (#900)
- Rename `pointerover` event to `pointermove` (#897)
- Only focus focusable nodes (#884)
- `a11y_auto_focus` (#878)
- Focus-based keyboard events (#877)
- Rename a11y attributes (#869)
- Optional uncontrolled accessibility IDs (#867)
- Rename `mouseover` to `mousemove` (#865)
- infer accesskit properties from node state (#855)
- Testing events utils (#864)
- Improved nodes events states (#859)
- Incremental Accessibility Tree (#853)
- `spacing` attribute (#834)
- `PluginHandle` (#793)
- Ignore unnecessary dioxus vdom mutations (#821)
- Avoid copying images when rendering (#808)
- Add missing gradient functions (#776)
- Support `space-between`/`space-around`/`space-evenly` alignments (#758)
- Use System fonts (#661)
- `highlight_mode` attribute (#704)
- Built-in vertical alignment for text (#701)
- Expose scale factor (#607)
- Reactive scale factor (#606)
- `winit` v0.30.0 + `glutin-winit` v0.5.0 + `accesskit` v0.14.0 + `accesskit_winit` v0.20.0  (#598)
- Revamp internal text selection (#647)
- Reactive Window data (#637)
- Reactive Platform data (#635)
- `use_preferred_theme` (#631)

### Fixed

- Handle reordedering of keyed children (#1015)
- Use paragraph's full width if text align is non-start (#998)
- Traverse layout-mutated children with no drawing area in compositor (#971)
- Skip updated but also removed accessibility nodes (#964)
- Use text_overflow value for paragraph's ellipsis
- Update the incremental removal of accessible nodes (#942)
- Check the default text align for expanded texts
- Use the real text height for layout (#932)
- Layout references not triggering (#934)
- Filter enter events properly, regression of #895 (#896)
- Use collateral event to check if event is allowed (#890)
- Proper incremental redraws for elements with outer or center borders (#879)
- Proper full render when selecting nodes in devtools
- Do a fullrerender when a devtools node is selected
- Show missing attributes in devtools (#801)
- Prevent opacity from clipping the node bounds (#764)
- Consider corner radius for events and overflow clipping (#768)
- Fix `unfocus` of accesibility nodes (#755)
- Skip DOM Nodes loaded in the same mutations run (#744)
- Invalidate layout of modified text nodes
- *(deps)* update all non-major dependencies (#578)
- Proper accessibility reactivity (#648)
- Fix performance dropping rapidly after selecting a text for some time (#624)
- Out of sync element ids on events (#609)

### Other

- add support for `fill` in `svg` (#797)
- Remove `DirtyNodesResults` from `native-core` (#999)
- Fmt and fix clippy warnings from 1.82
- Split core render functions (#937)
- Don't expand the label/paragraph width when using text aligns
- Be more generous for rotated dirty areas
- Accessibility logs
- Add opengl_rtt example. ([#813](https://github.com/marc2332/freya/pull/813))
- *(deps)* update all non-major dependencies (#749)
- Revert part of #731
- Rethink mutations writer (#731)
- Reorganize `freya-renderer` (#715)
- Rename node states to follow an unified naming (#713)
- Move rendering to `freya-core` (#712)
- `rustfmt.toml` (#689)
- Run clippy in tests and examples
- process all queued keyboard events at once (#629)
- release-plz.toml
- Only release crates under /crates
