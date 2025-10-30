//! # UI
//!
//! Freya uses a [declarative](https://en.wikipedia.org/wiki/Declarative_programming) model for the UI.
//!
//! For example, this is how the app component would look like in Freya:
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rect()
//!         .background((255, 0, 0))
//!         .width(Size::fill())
//!         .height(Size::px(100.))
//!         .on_mouse_up(|_| println!("Clicked!"))
//!         .into()
//! }
//! ```
//!
//! Notice that the `app` component is returning an [`Element`](freya_core::prelude::Element), this is because `rect()` gives you a [`Rect`](freya_core::elements::rect::Rect) which implements `Into<Element>`. So, in other words, the [`Element`](freya_core::prelude::Element) contains the UI of that component.
//! Every time the component function reruns a new UI is created and later diffed by Freya internally.
