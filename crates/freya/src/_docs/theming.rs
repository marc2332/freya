//! # Themes
//!
//! All the built-in components of Freya support themes, so if you find yourself wanting to tweak a certain style attribute of a component you might want to see if it can be changed through a theme.
//!
//! ### ThemeProvider
//!
//! You can pass a ThemeProvider to your whole app or maybe just a part of it by using the ThemeProvider component.
//!
//! Example:
//!
//! ```rust
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
//!         /// All the components descendant of this ThemeProvider will inherit the Custom Theme
//!         /// Again, this could be your whole app or maybe just a small part.
//!         ThemeProvider {
//!             theme: CUSTOM_THEME,
//!             rect {
//!                 width: "100%",
//!                 height: "100%",
//!                 Button {
//!                     label {
//!                         "Cancel"
//!                     }
//!                 }
//!             }
//!         }
//!     )
//! }
//! ```
//!
//! ### `theme` prop
//!
//! Most of the components also support being tweaked via their `theme` prop and with the help of the `theme_with` macro.
//!
//! Example:
//!
//! ```rust
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
