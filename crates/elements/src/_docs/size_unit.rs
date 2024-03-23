//! ### Size Units
//!
//! #### Auto
//! Will use it's inner children as size, so in this case, the `rect` width will be equivalent to the width of `label`:
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx!(
//!         rect {
//!             width: "auto",
//!             height: "33",
//!             label {
//!                 "hello!"
//!             }
//!         }
//!     )
//! }
//! ```
//!
//! ##### Logical pixels
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx!(
//!         rect {
//!             width: "50",
//!             height: "33"
//!         }
//!     )
//! }
//! ```
//!
//! ##### Parent percentage
//! Relative percentage to the parent equivalent value.
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx!(
//!         rect {
//!             width: "50%", // Half the parent
//!             height: "75%" // 3/4 the parent
//!         }
//!     )
//! }
//! ```
//!
//! ##### `calc()`
//!
//! For more complex logic you can use the `calc()` function.
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx!(
//!         rect {
//!             width: "calc(33% - 60 + 15%)", // (1/3 of the parent minus 60) plus 15% of parent
//!             height: "calc(100% - 10)" // 100% of the parent minus 10
//!         }
//!     )
//! }
//! ```
//!
//! #### fill
//! Use the remaining available space from the parent area:
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx!(
//!         rect {
//!             width: "100%",
//!             height: "100%",
//!             rect {
//!                 height: "200",
//!                 width: "100%",
//!             }
//!             rect {
//!                 height: "fill", // This is the same as calc(100% - 200)
//!                 width: "100%",
//!             }
//!         }
//!     )
//! }
//! ```
//!
//! #### fill-min
//! Will have the same size of the biggest sibling element inside a container who has `content: fit`.
//! For an example, see `content`.
//!
//! #### Viewport percentage
//! Relative percentage to the viewport (Window) equivalent value.
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! fn app() -> Element {
//!     rsx!(
//!         rect {
//!             width: "200",
//!             height: "200",
//!             rect {
//!                 height: "25v", // 25% of the window height no matter what height the parent has.
//!                 width: "100%",
//!             }
//!         }
//!     )
//! }
//! ```
