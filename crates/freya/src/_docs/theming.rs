//! # Themes
//!
//! All the built-in components of Freya support themes, so if you find yourself wanting to tweak a certain style attribute of a component you might want to see if it can be changed through a theme.
//!
//! ## `ThemeProvider` component
//!
//! You can wrap your whole app in a [ThemeProvider](freya_components::ThemeProvider) or maybe just a part of it.
//!
//! ### Example
//!
//! ```rust
//! # use freya::prelude::*;
//! // A custom theme based on the Light Theme that simply tweaks some parts of the Button theme.
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
//!         // All the components descendant of this ThemeProvider will inherit the Custom Theme
//!         // Again, this could be your whole app or maybe just a small part.
//!         ThemeProvider {
//!             theme: CUSTOM_THEME,
//!             Button {
//!                 label {
//!                     "Cancel"
//!                 }
//!             }
//!         }
//!     )
//! }
//! ```
//!
//! ## `use_init_theme` hook
//!
//! This is like `ThemeProvider` but as a hook. You can call this in your root component or somewhere else.
//!
//! ```rust
//! # use freya::prelude::*;
//! // A custom theme based on the Light Theme that simply tweaks some parts of the Button theme.
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
//!     use_init_theme(|| CUSTOM_THEME);
//!
//!     rsx!(
//!         Button {
//!             label {
//!                 "Cancel"
//!             }
//!         }
//!     )
//! }
//! ```
//!
//! ## `theme` prop
//!
//! Most of the components also support being tweaked via their `theme` prop and with the help of the `theme_with` macro.
//!
//! ### Example
//!
//! ```rust
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx!(
//!         Button {
//!             // You could pass the whole theme or maybe just a part of it
//!             theme: theme_with!(ButtonTheme {
//!                 background: "red".into(),
//!                 width: "200".into(),
//!             }),
//!             label {
//!                 "Cancel"
//!             }
//!         }
//!     )
//! }
//! ```
