# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-hooks-v0.2.1...freya-hooks-v0.3.0) - 2024-12-24

### Added

- `ResizableContainer` (#752)
- New `AnimatedPosition` component (#1013)
- `on_deps_change` method for animations (#977)
- Allow passing state context in freya-testing (#981)
- Button variants (#952)
- add attributes for most AccessKit properties (#882)
- Improve scrollbar colors for Dark theme
- Keyboard navigation for `Checkbox` (#926)
- Unified colors theming (#914)
- Add `UseEditable::new_in_hook` for manual creation of editable content (#933)
- Only focus focusable nodes (#884)
- `a11y_auto_focus` (#878)
- Focus-based keyboard events (#877)
- Rename a11y attributes (#869)
- Optional uncontrolled accessibility IDs (#867)
- Rename `mouseover` to `mousemove` (#865)
- Ergonomic improvements in ScrollView (#858)
- Testing events utils (#864)
- Refreshed theme colors (#856)
- Nicer Switch (#848)
- Incremental Accessibility Tree (#853)
- `spacing` attribute (#834)
- `PluginHandle` (#793)
- Dropdown layout improvements and new width theme option
- Move `shadow` of `Input` to `InputTheme` (#781)
- `BottomTab` component (#747)
- Simplify return type of `use_animation` (#748)
- Improve layout of `Button` component
- Use System fonts (#661)
- Signal-based reactivity for `use_canvas` (#693)
- Expose scale factor (#607)
- `winit` v0.30.0 + `glutin-winit` v0.5.0 + `accesskit` v0.14.0 + `accesskit_winit` v0.20.0  (#598)
- Replace selected text with the new insert (#678)
- Optionally allow inserting tabs as spaces (#664)
- Tab and Tabsbar components (#673)
- `onpress` event for `Button` (#601)
- Select all text (#652)
- Revamp internal text selection (#647)
- Reactive Platform data (#635)
- `use_preferred_theme` (#631)
- Remove text with Delete (#644)
- Text dragging with shift and cursor (#642)
- Delete text selection with backspace (#640)
- Improved special text editing support (#622)
- `WithWindow` event (#626)
- `placeholder` for Input (#616)
- Close app with `use_platform` (#613)
- Add window drag area (#597)

### Fixed

- Update the `use_editable` tests to match github ci (#1012)
- Add some missing  theme values
- Fix text selection movement of the cursor
- Use the real text height for layout (#932)
- Various fixes when editing utf16-encoded text (#901)
- Proper support for keyboard navigation for Radio (#880)
- Proper drop of unused assets in asset cacher (#868)
- Use hotfix patch for nokhwa
- Last frame of animations was not always applied (#798)
- Support alpha channel in AnimColor (#771)
- Consider corner radius for events and overflow clipping (#768)
- Prevent crash on keyboard navigation with empty text (#706)
- Store cached assets in Root Scope (#668)
- Stop at line length when navigating with keyboard arrows in text
- Proper accessibility reactivity (#648)
- Fix performance dropping rapidly after selecting a text for some time (#624)

### Other

- Replace dioxus-sdk with dioxus-clipboard (#973)
- Add missing 0.2 docs (#916)
- Fix clippy warnings in use_init_native_platforms tests
- Clean up
- cargo clippy
- Add opengl_rtt example. ([#813](https://github.com/marc2332/freya/pull/813))
- Small fixes and clean up of internal code
- New shader editor example
- `rustfmt.toml` (#689)
- Use single position cursor (#653)
- Add tests for shift + click text selection
- Fix shift+click selection in virtualized editors
- Enable a `use_theme` doc test
- Fix text selection in some cases
- release-plz.toml
- Only release crates under /crates
