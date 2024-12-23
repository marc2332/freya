# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-renderer-v0.2.1...freya-renderer-v0.3.0) - 2024-12-23

### Added

- Event Loop Builder hook (#944)
- `LaunchConfig::with_visible` (#935)
- Custom scale factor shortcuts (#931)
- `a11y_auto_focus` (#878)
- Focus-based keyboard events (#877)
- Optional uncontrolled accessibility IDs (#867)
- Rename `mouseover` to `mousemove` (#865)
- Testing events utils (#864)
- Improved nodes events states (#859)
- Graphics Drivers (#822)
- Incremental Accessibility Tree (#853)
- `PluginHandle` (#793)
- Allow custom Tokio Runtimes (#765)
- Use System fonts (#661)
- Skia-safe v0.75 (#716)
- Only send keydowns when the window is focused (#705)
- `highlight_mode` attribute (#704)
- Built-in vertical alignment for text (#701)
- Expose scale factor (#607)
- Reactive scale factor (#606)
- `winit` v0.30.0 + `glutin-winit` v0.5.0 + `accesskit` v0.14.0 + `accesskit_winit` v0.20.0  (#598)
- Tree-like explorer for devtools (#684)
- More reliable devtools (#667)
- Queued focus (#650)
- Revamp internal text selection (#647)
- Reactive Window data (#637)
- Reactive Platform data (#635)
- `use_preferred_theme` (#631)
- `WithWindow` event (#626)
- Close app with `use_platform` (#613)
- Add window drag area (#597)

### Fixed

- Create accesskit adapter before making window visible (#967)
- Scale with window's scaled actor the areas requested for invalid… (#912)
- Proper full render when selecting nodes in devtools
- Do a fullrerender when a devtools node is selected
- Small devtool fixes
- Use `ImageReader` for icon loading in windows
- Prevent opacity from clipping the node bounds (#764)
- Consider corner radius for events and overflow clipping (#768)
- Only send left mouseover event when not clicking the mouse (#753)
- Require to pass both the width and height for window size at once when desired (#757)
- Call `on_setup` hook
- *(deps)* update all non-major dependencies (#578)
- Fix some inconsitencies with the 0.30 update (#696)
- Proper accessibility reactivity (#648)
- Out of sync element ids on events (#609)

### Other

- Remove `accesskit` window rendering workaround (#930)
- Resolve conflicts
- Add opengl_rtt example. ([#813](https://github.com/marc2332/freya/pull/813))
- Adjust root element height of user app in devtools
- ExitApp command was not working
- Rethink mutations writer (#731)
- Simplify the `VirtualDOM` polling (#729)
- Clean up some code from `freya-renderer`
- Only make window visible once built
- Reorganize `freya-renderer` (#715)
- Move rendering to `freya-core` (#712)
- `rustfmt.toml` (#689)
- process all queued keyboard events at once (#629)
- release-plz.toml
- Only release crates under /crates
- Fix typo on `with_default_font`
