# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-components-v0.2.1...freya-components-v0.3.0) - 2024-12-23

### Added

- Prepare New Docs (#788)
- `ResizableContainer` (#752)
- New `OverflowContent` component (#1011)
- New `AnimatedPosition` component (#1013)
- `on_deps_change` method for animations (#977)
- `text_height` attribute to control the height behavior of text (#976)
- Horizontal scroll for `Input` (#949)
- Button variants (#952)
- add attributes for most AccessKit properties (#882)
- Add x and y getters for the scroll controller
- multiple borders (#889)
- Keyboard navigation for `Checkbox` (#926)
- Unified colors theming (#914)
- Ajust the custom layers of some built-in-components (#928)
- Move sliders with keyboard (#917)
- Vertical `direction` for `Slider` (#910)
- `import_image` (#899)
- `TooltipContainer` (#900)
- Small improvements in `SnackBar`
- Only focus focusable nodes (#884)
- `a11y_auto_focus` (#878)
- Focus-based keyboard events (#877)
- Optional `onclick` event handle for `Tab`
- `invert_scroll_wheel` for `ScrollView` and `VirtualScrollView`
- Rename a11y attributes (#869)
- Optionally hide original drag zone children while dragging (#871)
- Rename `mouseover` to `mousemove` (#865)
- Ergonomic improvements in ScrollView (#858)
- Testing events utils (#864)
- Improved nodes events states (#859)
- Refreshed theme colors (#856)
- Nicer Switch (#848)
- Optional size for import_svg
- Support extra routes in `ActivableRoute`
- website example (#839)
- `spacing` attribute (#834)
- `PluginHandle` (#793)
- Dropdown layout improvements and new width theme option
- Support percentage of auto in layout (#784)
- `import_svg` macro (#790)
- Scroll controller (#772)
- Move `shadow` of `Input` to `InputTheme` (#781)
- Panic when an attribute has a wrongly-formatted value, but only in debug builds to easily spot bugs (#759)
- Change the wheels scroll based on the direction (#751)
- `BottomTab` component (#747)
- Animated router transitions (#745)
- Improve layout of `Button` component
- Render one more item in VirtualScrollView for smooth scrolling (#723)
- Use System fonts (#661)
- Signal-based reactivity for `use_canvas` (#693)
- Expose scale factor (#607)
- Tree builder utilities for components (#681)
- Tab and Tabsbar components (#673)
- Support nested routes in `ActivableRoute` (#675)
- Add backward compatibility for onclick event handler in Button
- `onpress` event for `Button` (#601)
- `use_preferred_theme` (#631)
- Text dragging with shift and cursor (#642)
- Improved special text editing support (#622)
- `WithWindow` event (#626)
- `placeholder` for Input (#616)
- Animated `VirtualScrollView` (#604)
- Add window drag area (#597)
- Close popup with esc key (#594)

### Fixed

- Round ScrollView size (#972)
- Update checkbox tests
- Use the real text height for layout (#932)
- Clamp progress bar progress (#927)
- Various fixes when editing utf16-encoded text (#901)
- Use collateral event to check if event is allowed (#890)
- Proper support for keyboard navigation for Radio (#880)
- Small devtool fixes
- Better input click handling
- Force the dropdown items vertically (#827)
- Consider corner radius for events and overflow clipping (#768)
- *(deps)* update all non-major dependencies (#578)
- Small fixes for Tabs
- Proper accessibility reactivity (#648)
- `WindowDragArea` component hanging on non-left click. (#625)

### Other

- add support for `fill` in `svg` (#797)
- Remove `text_align` from `Button`, `Tab` and `BottomTab`
- Replace dioxus-sdk with dioxus-clipboard (#973)
- Add missing 0.2 docs (#916)
- Clean up readme `Tile`
- Remove unnecessary border wrappers of Radio and Checkbox (#955)
- Fix dropdown test
- Fix snackbar tests
- Clean up log
- Fix drag and drop tests
- Update drag_drop.rs example
- Fix cargo clippy
- Add opengl_rtt example. ([#813](https://github.com/marc2332/freya/pull/813))
- Increase Sidebar width
- Clean up Tabs components
- `rustfmt.toml` (#689)
- Run clippy in tests and examples
- release-plz.toml
- Only release crates under /crates
- Rust 1.78 (#600)
