//! # Animating
//!
//! Freya comes with `use_animation`, a hook you can use to easily animate your elements.
//!
//! You can animate numeric values (e.g width, padding, rotation, offsets) or also colors.
//! You can specify the duration, the easing functin and what type of easing you want.
//!
//! Here is an example that animates a value from `0.0` to `100.0` in `50` milliseconds.
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//!
//! fn main() {
//!     launch(app);
//! }
//!
//! fn app() -> Element {
//!     let animation = use_animation(|ctx| ctx.with(AnimNum::new(0., 100.).time(50)));
//!
//!     let animations = animation.read();
//!     let width = animations.get().read().as_f32();
//!
//!     use_hook(move || {
//!         // Start animation as soon as this component runs.
//!         animation.read().start();
//!     });
//!
//!     rsx!(
//!         rect {
//!             width: "{width}",
//!             height: "100%",
//!             background: "blue"
//!         }
//!     )
//! }
//! ```
//!
//! You are not limited to just one animation per call, you can have as many as you want.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     let animation = use_animation(|ctx| {
//!         (
//!             ctx.with(AnimNum::new(0., 100.).time(50)),
//!             ctx.with(AnimColor::new("red", "blue").time(50))
//!         )
//!     });
//!
//!     let animations = animation.read();
//!     let (width, color) = animations.get();
//!
//!     use_hook(move || {
//!         animation.read().start();
//!     });
//!
//!     rsx!(
//!         rect {
//!             width: "{width.read().as_f32()}",
//!             height: "100%",
//!             background: "{color.read().as_string()}"
//!         }
//!     )
//! }
