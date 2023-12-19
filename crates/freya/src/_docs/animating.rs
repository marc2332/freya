//! # Animating
//!
//! Freya provides you with two hooks to help you animate your components.
//!
//! ### `use_animation`
//!
//! This is a simple hook that lets you animate a certain value from an `initial`
//! value to a `final` value, in a given `duration` of time.
//! There are a few animations that you can select:
//!
//! - Linear
//! - EaseIn
//! - EaseInOut
//! - BounceIns
//!
//! Here is an example that animates a value from `0.0` to `100.0` in `50` milliseconds,
//! using the `linear` animation.
//!
//! ```rust, no_run
//! fn main() {
//!     launch(app);
//! }
//!
//!  fn app(cx: Scope) -> Element {
//!     let animation = use_animation(cx, || 0.0);
//!
//!     let progress = animation.value();
//!
//!     use_memo(cx, (), move |_| {
//!         animation.start(Animation::new_linear(0.0..=100.0, 50));
//!     });
//!
//!     render!(rect {
//!         width: "{progress}",
//!     })
//! }
//! ```
//!
//! ### `use_animation_transition`
//!
//! This hook lets you group a set of animations together with a certain `animation` and a given `duration`.
//! You can specify a set of dependencies that re-runs the animation callback.
//!
//! You have these animations:
//!
//! - Linear
//! - EaseIn
//! - EaseInOut
//! - BounceIns
//!
//! Here is an example that animates a `size` and a color in `200` milliseconds, using the `new_sine_in_out` animation.
//!
//! ```rust, no_run
//! fn main() {
//!     launch(app);
//! }
//!
//! const TARGET: f64 = 500.0;
//!
//! fn app(cx: Scope) -> Element {
//!     let animation = use_animation_transition(cx, TransitionAnimation::new_sine_in_out(200), (), || {
//!         vec![
//!             Animate::new_size(0.0, TARGET),
//!             Animate::new_color("rgb(33, 158, 188)", "white"),
//!         ]
//!     });
//!
//!     let size = animation.get(0).unwrap().as_size();
//!     let background = animation.get(1).unwrap().as_color();
//!
//!     let onclick = move |_: MouseEvent| {
//!         if size == 0.0 {
//!             animation.start();
//!         } else if size == TARGET {
//!             animation.reverse();
//!         }
//!     };
//!
//!     render!(
//!         rect {
//!             overflow: "clip",
//!             background: "black",
//!             width: "100%",
//!             height: "100%",
//!             offset_x: "{size}",
//!             rect {
//!                 height: "100%",
//!                 width: "200",
//!                 background: "{background}",
//!                 onclick: onclick,
//!             }
//!         }
//!     )
//! }
//! ```
