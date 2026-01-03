//! # i18n
//!
//! You may add i18n (localization) support to your Freya app by using [**freya-i18n**](https://crates.io/crates/freya-i18n) crate.
//! You can also enable its reexport by turning on the `i18n` feature in `freya`.
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
//! // main.rs
//! #[derive(PartialEq)]
//! struct Body;
//!
//! impl Render for Body {
//!     fn render(&self) -> impl IntoElement {
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
//!     }
//! }
//!
//! fn app() -> impl IntoElement {
//!     // Initialize our i18n config
//!     use_init_i18n(|| {
//!         I18nConfig::new(langid!("en-US"))
//!             .with_locale(Locale::new_static(
//!                 langid!("en-US"),
//!                 include_str!("../../../examples/i18n/en-US.ftl"),
//!             ))
//!             .with_locale(Locale::new_dynamic(
//!                 langid!("es-ES"),
//!                 "../../../examples/i18n/es-ES.ftl",
//!             ))
//!     });
//!
//!     Body
//! }
//! ```

mod error;
pub mod i18n;
pub mod i18n_macro;

pub use fluent;
pub use unic_langid;

pub mod prelude {
    pub use unic_langid::{
        LanguageIdentifier,
        lang,
        langid,
    };

    pub use crate::{
        error::Error as DioxusI18nError,
        i18n::*,
        t,
        te,
        tid,
    };
}
