//! # i18n
//!
//! You may add i18n (localization) support to your Freya app by using the [**freya-i18n**](https://crates.io/crates/freya-i18n) crate.
//! You can also enable its reexport by turning on the `i18n` feature in freya.
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
//! # use freya_i18n::prelude::*;
//!
//! // main.rs
//!
//! fn main() {
//!     # return;
//!     launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
//! }
//!
//! #[derive(PartialEq)]
//! struct Body;
//!
//! impl Render for Body {
//!     fn render(&self) -> Element {
//!         // Access to the i18n state
//!         let mut i18n = I18n::get();
//!
//!         // Update the current language
//!         let change_to_english = move |_| i18n.set_language(langid!("en-US"));
//!         let change_to_spanish = move |_| i18n.set_language(langid!("es-ES"));
//!
//!         rect()
//!           .expanded()
//!           .center()
//!           .child(
//!               rect()
//!                   .horizontal()
//!                   .child(Button::new().on_press(change_to_english).child("English"))
//!                   .child(Button::new().on_press(change_to_spanish).child("Spanish")),
//!           )
//!          .child(t!("hello_world"))
//!          .child(t!("hello", name: "Freya!"))
//!          .into()
//!     }
//! }
//!
//! fn app() -> Element {
//!     // Initialize our i18n config
//!     use_init_i18n(|| {
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
//!     Body.into()
//! }
//! ```
