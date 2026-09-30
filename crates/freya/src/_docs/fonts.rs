//! # Fonts
//!
//! Freya makes the fonts installed in your system available to your app, but you can also embed
//! fonts in your application, or load them while the application is running.
//!
//! ## Choosing a font
//!
//! Set the font family on an element with [`font_family`](freya_core::elements::extensions::TextStyleExt::font_family).
//! The family name must match the name of the installed font or the one you used to register it in your Freya app.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! fn app() -> impl IntoElement {
//!     label()
//!         .font_size(24.)
//!         .font_family("Noto Sans")
//!         .text("Text using Noto Sans")
//! }
//! ```
//!
//! A font family applies to the element and its descendants unless a descendant chooses another
//! family. This makes it possible to set an application-wide family on the root element.
//!
//! ## Embedding a font
//!
//! Register a font before launching the application with [`LaunchConfig::with_font`](freya_winit::config::LaunchConfig::with_font).
//! The first argument is the name that will later be passed to `font_family`. The font bytes get embedded in the executable with `include_bytes!`:
//!
//! ```text
//! # use freya::prelude::*;
//! fn main() {
//!     launch(
//!         LaunchConfig::new()
//!             .with_font(
//!                 "My Font",
//!                 Bytes::from_static(include_bytes!("../assets/my-font.ttf")),
//!             )
//!             .with_default_font("My Font")
//!             .with_window(WindowConfig::new(app)),
//!     )
//! }
//! ```
//!
//! `with_default_font` makes the registered family the preferred font for elements that do not
//! specify a family. It does not replace the system fallback fonts, so text can still use fallback
//! fonts when the selected family does not contain a required glyph.
//!
//! You can register multiple font files under the same family name when the files represent
//! different styles or weights. Freya will use the matching face when text styling requests it.
//!
//! ## Loading a font dynamically
//!
//! [`Platform::load_font`](freya_core::platform::Platform::load_font) registers a font after the
//! application has started. The font is made available in all windows. Loading bytes from a file
//! is useful when the font is optional or selected by the user:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! fn load_user_font() {
//!     let font_data = match std::fs::read("./fonts/my-font.ttf") {
//!         Ok(font_data) => font_data,
//!         Err(error) => {
//!             eprintln!("Could not read font: {error}");
//!             return;
//!         }
//!     };
//!
//!     Platform::get().load_font("My Font", font_data);
//! }
//!
//! fn app() -> impl IntoElement {
//!     Button::new()
//!         .on_press(|_| load_user_font())
//!         .child("Load font")
//! }
//! ```
//!
//! After loading the font, use the registered name on an element:
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! fn app() -> impl IntoElement {
//!     label()
//!         .font_family("My Font")
//!         .text("This uses the dynamically loaded font")
//! }
//! ```
//!
//! A font loaded at runtime is not persisted by Freya. Load it again each time the application
//! starts if it is needed on every launch.
//!
//! ## Web applications
//!
//! Web applications register fonts with [`WebConfig::with_font`](freya_web::WebConfig::with_font)
//! and can set their default families with [`WebConfig::with_default_fonts`](freya_web::WebConfig::with_default_fonts):
//!
//! ```text
//! freya::web::launch(
//!     freya::web::WebConfig::new(app)
//!         .with_font("My Font", MY_FONT)
//!         .with_default_fonts(vec!["My Font".into()]),
//! );
//! ```
//!
//! The font family name used in `font_family` must be the same name passed to `with_font`.
