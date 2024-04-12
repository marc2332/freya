//! # Theming
//!
//! <div class="warning">⚠️ As of 2023-12-19, extending the base theme is not supported.</div>
//!
//! Freya has built-in support for Theming.
//!
//! ### Accessing the current theme
//!
//! You can access the current theme via the `use_get_theme` hook.
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx!(
//!         ThemeProvider {
//!             Component { }
//!         }
//!     )
//! }
//!
//! #[allow(non_snake_case)]
//! fn Component() -> Element {
//!     let theme = use_get_theme();
//!
//!     let button_theme = &theme.button;
//!
//!     rsx!(
//!         rect {
//!             background: "{button_theme.background}",
//!         }
//!     )
//! }
//! ```
//!
//! ## Custom default theme
//!
//! By default, the selected theme is `LIGHT_THEME`. You can use the alternative, `DARK_THEME`.
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx!(
//!         ThemeProvider {
//!             theme: LIGHT_THEME,
//!             Component { }
//!         }
//!     )
//! }
//!
//! #[allow(non_snake_case)]
//! fn Component() -> Element {
//!     let theme = use_get_theme();
//!
//!     let button_theme = &theme.button;
//!
//!     rsx!(
//!         rect {
//!             background: "{button_theme.background}",
//!         }
//!     )
//! }
//! ```
//!
//! ## Change the theme
//!
//! Changing the selected theme at runtime is possible by using the `use_theme` hook.
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx!(
//!         ThemeProvider {
//!             Component { }
//!         }
//!     )
//! }
//!
//! #[allow(non_snake_case)]
//! fn Component() -> Element {
//!     let mut theme = use_theme();
//!
//!     let onclick = move |_| {
//!         *theme.write() = LIGHT_THEME;
//!     };
//!
//!     rsx!(
//!         Button {
//!             onclick,
//!             label {
//!                 "Use Light theme"
//!             }
//!         }
//!     )
//! }
//! ```
//!
//! ## Change theme for an individual component
//!
//! Most built-in components have their own theme "override."
//! You can specify values to override like this:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx! {
//!         Button {
//!             theme: ButtonThemeWith {
//!                 background: Some("blue".into()),
//!                 font_theme: Some(FontThemeWith {
//!                     color: Some("white".into()),
//!                     ..Default::default()
//!                 }),
//!                 ..Default::default()
//!             },
//!             label { "I'm blue now" }
//!         }
//!     }
//! }
//! ```
//!
//! You need to use a different "type" of theme.
//! In the "ThemeWith" structs, each field is optional, so that the component knows what to override and
//! what to keep.
//! Also, you need to spread `..Default::default`, to make all the other fields `None`.
//!
//! To make this less verbose, you can use the `theme_with!` macro:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx! {
//!         Button {
//!             theme: theme_with!(ButtonTheme {
//!                 background: "blue".into(),
//!                 font_theme: theme_with!(FontTheme {
//!                     color: "white".into(),
//!                 }),
//!             }),
//!             label { "I'm blue now" }
//!         }
//!     }
//! }
//! ```
//!
//! As you can see, it removes the need for the "With" suffix, because that is already in the macro name.
//! More importantly, though, it wraps each field in a `Some`, and adds the spread.
//!
//! ## Custom theme
//!
//! You can build themes from scratch or extended from others, like here with `LIGHT_THEME`:
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! const CUSTOM_THEME: Theme = Theme {
//!     button: ButtonTheme {
//!         background: Cow::Borrowed("rgb(230, 0, 0)"),
//!         hover_background: Cow::Borrowed("rgb(150, 0, 0)"),
//!         font_theme: FontTheme {
//!             color: Cow::Borrowed("white"),
//!         },
//!         ..LIGHT_THEME.button
//!     },
//!     ..LIGHT_THEME
//! };
//!
//! fn app() -> Element {
//!     rsx!(
//!         ThemeProvider {
//!             theme: CUSTOM_THEME,
//!             rect {
//!                 width: "100%",
//!                 height: "100%",
//!                 Button {
//!                     label {
//!                         "Report"
//!                     }
//!                 }
//!             }
//!         }
//!     )
//! }
//! ```
