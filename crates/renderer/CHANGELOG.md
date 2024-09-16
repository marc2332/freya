# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-renderer-v0.2.1...freya-renderer-v0.3.0) - 2024-09-16

### Added

- `a11y_auto_focus` ([#878](https://github.com/marc2332/freya/pull/878))
- Focus-based keyboard events ([#877](https://github.com/marc2332/freya/pull/877))
- Optional uncontrolled accessibility IDs ([#867](https://github.com/marc2332/freya/pull/867))
- Rename `mouseover` to `mousemove` ([#865](https://github.com/marc2332/freya/pull/865))
- Testing events utils ([#864](https://github.com/marc2332/freya/pull/864))
- Improved nodes events states ([#859](https://github.com/marc2332/freya/pull/859))
- Graphics Drivers ([#822](https://github.com/marc2332/freya/pull/822))
- Incremental Accessibility Tree ([#853](https://github.com/marc2332/freya/pull/853))
- `PluginHandle` ([#793](https://github.com/marc2332/freya/pull/793))
- Allow custom Tokio Runtimes ([#765](https://github.com/marc2332/freya/pull/765))
- Use System fonts ([#661](https://github.com/marc2332/freya/pull/661))
- Skia-safe v0.75 ([#716](https://github.com/marc2332/freya/pull/716))
- Only send keydowns when the window is focused ([#705](https://github.com/marc2332/freya/pull/705))
- `highlight_mode` attribute ([#704](https://github.com/marc2332/freya/pull/704))
- Built-in vertical alignment for text ([#701](https://github.com/marc2332/freya/pull/701))
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

- Proper full render when selecting nodes in devtools
- Do a fullrerender when a devtools node is selected
- Small devtool fixes
- Use `ImageReader` for icon loading in windows
- Prevent opacity from clipping the node bounds ([#764](https://github.com/marc2332/freya/pull/764))
- Consider corner radius for events and overflow clipping ([#768](https://github.com/marc2332/freya/pull/768))
- Only send left mouseover event when not clicking the mouse ([#753](https://github.com/marc2332/freya/pull/753))
- Require to pass both the width and height for window size at once when desired ([#757](https://github.com/marc2332/freya/pull/757))
- Call `on_setup` hook
- *(deps)* update all non-major dependencies ([#578](https://github.com/marc2332/freya/pull/578))
- Fix some inconsitencies with the 0.30 update ([#696](https://github.com/marc2332/freya/pull/696))
- Proper accessibility reactivity ([#648](https://github.com/marc2332/freya/pull/648))
- Out of sync element ids on events ([#609](https://github.com/marc2332/freya/pull/609))

### Other

- Resolve conflicts
- Add opengl_rtt example. ([#813](https://github.com/marc2332/freya/pull/813))
- Adjust root element height of user app in devtools
- ExitApp command was not working
- Rethink mutations writer ([#731](https://github.com/marc2332/freya/pull/731))
- Simplify the `VirtualDOM` polling ([#729](https://github.com/marc2332/freya/pull/729))
- Clean up some code from `freya-renderer`
- Only make window visible once built
- Reorganize `freya-renderer` ([#715](https://github.com/marc2332/freya/pull/715))
- Move rendering to `freya-core` ([#712](https://github.com/marc2332/freya/pull/712))
- `rustfmt.toml` ([#689](https://github.com/marc2332/freya/pull/689))
- process all queued keyboard events at once ([#629](https://github.com/marc2332/freya/pull/629))
- release-plz.toml
- Only release crates under /crates
- Fix typo on `with_default_font`
