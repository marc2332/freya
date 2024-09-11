# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/marc2332/freya/compare/freya-components-v0.2.1...freya-components-v0.3.0) - 2024-09-11

### Added

- Rename `mouseover` to `mousemove` ([#865](https://github.com/marc2332/freya/pull/865))
- Ergonomic improvements in ScrollView ([#858](https://github.com/marc2332/freya/pull/858))
- Testing events utils ([#864](https://github.com/marc2332/freya/pull/864))
- Improved nodes events states ([#859](https://github.com/marc2332/freya/pull/859))
- Refreshed theme colors ([#856](https://github.com/marc2332/freya/pull/856))
- Nicer Switch ([#848](https://github.com/marc2332/freya/pull/848))
- Optional size for import_svg
- Support extra routes in `ActivableRoute`
- website example ([#839](https://github.com/marc2332/freya/pull/839))
- `spacing` attribute ([#834](https://github.com/marc2332/freya/pull/834))
- `PluginHandle` ([#793](https://github.com/marc2332/freya/pull/793))
- Dropdown layout improvements and new width theme option
- Support percentage of auto in layout ([#784](https://github.com/marc2332/freya/pull/784))
- `import_svg` macro ([#790](https://github.com/marc2332/freya/pull/790))
- Scroll controller ([#772](https://github.com/marc2332/freya/pull/772))
- Move `shadow` of `Input` to `InputTheme` ([#781](https://github.com/marc2332/freya/pull/781))
- Panic when an attribute has a wrongly-formatted value, but only in debug builds to easily spot bugs ([#759](https://github.com/marc2332/freya/pull/759))
- Change the wheels scroll based on the direction ([#751](https://github.com/marc2332/freya/pull/751))
- `BottomTab` component ([#747](https://github.com/marc2332/freya/pull/747))
- Animated router transitions ([#745](https://github.com/marc2332/freya/pull/745))
- Improve layout of `Button` component
- Render one more item in VirtualScrollView for smooth scrolling ([#723](https://github.com/marc2332/freya/pull/723))
- Use System fonts ([#661](https://github.com/marc2332/freya/pull/661))
- Signal-based reactivity for `use_canvas` ([#693](https://github.com/marc2332/freya/pull/693))
- Expose scale factor ([#607](https://github.com/marc2332/freya/pull/607))
- Tree builder utilities for components ([#681](https://github.com/marc2332/freya/pull/681))
- Tab and Tabsbar components ([#673](https://github.com/marc2332/freya/pull/673))
- Support nested routes in `ActivableRoute` ([#675](https://github.com/marc2332/freya/pull/675))
- Add backward compatibility for onclick event handler in Button
- `onpress` event for `Button` ([#601](https://github.com/marc2332/freya/pull/601))
- `use_preferred_theme` ([#631](https://github.com/marc2332/freya/pull/631))
- Text dragging with shift and cursor ([#642](https://github.com/marc2332/freya/pull/642))
- Improved special text editing support ([#622](https://github.com/marc2332/freya/pull/622))
- `WithWindow` event ([#626](https://github.com/marc2332/freya/pull/626))
- `placeholder` for Input ([#616](https://github.com/marc2332/freya/pull/616))
- Animated `VirtualScrollView` ([#604](https://github.com/marc2332/freya/pull/604))
- Add window drag area ([#597](https://github.com/marc2332/freya/pull/597))
- Close popup with esc key ([#594](https://github.com/marc2332/freya/pull/594))

### Fixed

- Small devtool fixes
- Better input click handling
- Force the dropdown items vertically ([#827](https://github.com/marc2332/freya/pull/827))
- Consider corner radius for events and overflow clipping ([#768](https://github.com/marc2332/freya/pull/768))
- *(deps)* update all non-major dependencies ([#578](https://github.com/marc2332/freya/pull/578))
- Small fixes for Tabs
- Proper accessibility reactivity ([#648](https://github.com/marc2332/freya/pull/648))
- `WindowDragArea` component hanging on non-left click. ([#625](https://github.com/marc2332/freya/pull/625))

### Other

- Fix cargo clippy
- Merge branch 'main' into feat/incremental-rendering
- Add opengl_rtt example. ([#813](https://github.com/marc2332/freya/pull/813))
- Increase Sidebar width
- Clean up Tabs components
- `rustfmt.toml` ([#689](https://github.com/marc2332/freya/pull/689))
- Run clippy in tests and examples
- release-plz.toml
- Only release crates under /crates
- Rust 1.78 ([#600](https://github.com/marc2332/freya/pull/600))
