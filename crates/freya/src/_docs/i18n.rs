//! # i18n
//!
//! You may add i18n (localization) support to your Freya app by using the [**dioxus-i18n**](https://github.com/dioxus-community/dioxus-i18n) crate.
//!
//! ```fluent
//! # en-US.ftl
//!
//! hello_world = Hello, World!
//! hello = Hello, {$name}!
//! ```
//!
//!
//! ```fluent
//! # es-ES.ftl
//!
//! hello_world = Hola, Mundo!
//! hello = Hola, {$name}!
//! ```
//!
//! ```rust
//! # use freya::prelude::*;
//! # use dioxus_i18n::{prelude::*, t};
//! # use dioxus_i18n::unic_langid::langid;
//!
//! // main.rs
//!
//! fn main() {
//!     # return;
//!     launch(app);
//! }
//!
//! #[allow(non_snake_case)]
//! fn Body() -> Element {
//!     // Access to the i18n state
//!     let mut i18n = i18n();
//!
//!     // Update the current language
//!     let change_to_english = move |_| i18n.set_language(langid!("en-US"));
//!     let change_to_spanish = move |_| i18n.set_language(langid!("es-ES"));
//!
//!     rsx!(
//!         rect {
//!             rect {
//!                 direction: "horizontal",
//!                 Button {
//!                     onpress: change_to_english,
//!                     label {
//!                         "English"
//!                     }
//!                 }
//!                 Button {
//!                     onpress: change_to_spanish,
//!                     label {
//!                         "Spanish"
//!                     }
//!                 }
//!             }
//!
//!             // Get and subscribe to these messages
//!             label { { t!("hello_world") } }
//!             label { { t!("hello", name: "Dioxus") } }
//!         }
//!     )
//! }
//!
//! fn app() -> Element {
//!     // Initialize our i18n config
//!     use_init_i18n(|| {
//!         # return I18nConfig::new(langid!("en-US"));
//!         I18nConfig::new(langid!("en-US"))
//!             .with_locale(Locale::new_static(
//!                 langid!("en-US"),
//!                 include_str!("./en-US.ftl"),
//!             ))
//!             .with_locale(Locale::new_dynamic(
//!                 langid!("es-ES"),
//!                 "./es-ES.ftl",
//!             ))
//!     });
//!
//!     rsx!(Body {})
//! }
//! ```
