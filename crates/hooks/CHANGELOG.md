# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2](https://github.com/marc2332/freya/compare/freya-hooks-v0.2.1...freya-hooks-v0.2.2) - 2024-08-10

### Added
- Move `shadow` of `Input` to `InputTheme` ([#781](https://github.com/marc2332/freya/pull/781))
- `BottomTab` component ([#747](https://github.com/marc2332/freya/pull/747))
- Simplify return type of `use_animation` ([#748](https://github.com/marc2332/freya/pull/748))
- Improve layout of `Button` component
- Use System fonts ([#661](https://github.com/marc2332/freya/pull/661))
- Signal-based reactivity for `use_canvas` ([#693](https://github.com/marc2332/freya/pull/693))
- Expose scale factor ([#607](https://github.com/marc2332/freya/pull/607))
- `winit` v0.30.0 + `glutin-winit` v0.5.0 + `accesskit` v0.14.0 + `accesskit_winit` v0.20.0  ([#598](https://github.com/marc2332/freya/pull/598))
- Replace selected text with the new insert ([#678](https://github.com/marc2332/freya/pull/678))
- Optionally allow inserting tabs as spaces ([#664](https://github.com/marc2332/freya/pull/664))
- Tab and Tabsbar components ([#673](https://github.com/marc2332/freya/pull/673))
- `onpress` event for `Button` ([#601](https://github.com/marc2332/freya/pull/601))
- Select all text ([#652](https://github.com/marc2332/freya/pull/652))
- Revamp internal text selection ([#647](https://github.com/marc2332/freya/pull/647))
- Reactive Platform data ([#635](https://github.com/marc2332/freya/pull/635))
- `use_preferred_theme` ([#631](https://github.com/marc2332/freya/pull/631))
- Remove text with Delete ([#644](https://github.com/marc2332/freya/pull/644))
- Text dragging with shift and cursor ([#642](https://github.com/marc2332/freya/pull/642))
- Delete text selection with backspace ([#640](https://github.com/marc2332/freya/pull/640))
- Improved special text editing support ([#622](https://github.com/marc2332/freya/pull/622))
- `WithWindow` event ([#626](https://github.com/marc2332/freya/pull/626))
- `placeholder` for Input ([#616](https://github.com/marc2332/freya/pull/616))
- Close app with `use_platform` ([#613](https://github.com/marc2332/freya/pull/613))
- Add window drag area ([#597](https://github.com/marc2332/freya/pull/597))

### Fixed
- Use hotfix patch for nokhwa
- Last frame of animations was not always applied ([#798](https://github.com/marc2332/freya/pull/798))
- Support alpha channel in AnimColor ([#771](https://github.com/marc2332/freya/pull/771))
- Consider corner radius for events and overflow clipping ([#768](https://github.com/marc2332/freya/pull/768))
- Prevent crash on keyboard navigation with empty text ([#706](https://github.com/marc2332/freya/pull/706))
- Store cached assets in Root Scope ([#668](https://github.com/marc2332/freya/pull/668))
- Stop at line length when navigating with keyboard arrows in text
- Proper accessibility reactivity ([#648](https://github.com/marc2332/freya/pull/648))
- Fix performance dropping rapidly after selecting a text for some time ([#624](https://github.com/marc2332/freya/pull/624))

### Other
- Small fixes and clean up of internal code
- New shader editor example
- `rustfmt.toml` ([#689](https://github.com/marc2332/freya/pull/689))
- Use single position cursor ([#653](https://github.com/marc2332/freya/pull/653))
- Add tests for shift + click text selection
- Merge branch 'main' of https://github.com/marc2332/freya
- Fix shift+click selection in virtualized editors
- Enable a `use_theme` doc test
- Fix text selection in some cases
- release-plz.toml
- Only release crates under /crates
