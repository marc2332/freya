//! # Events
//!
//! Events let your UI react to user input: a click, a key press, the cursor entering an
//! element, a scroll, a file drop, and so on.
//!
//! Events are attached to **elements** (`rect`, `label`, `paragraph`, ...), not to components.
//! Components are just a way to organize and reuse UI; the listeners are on the elements that
//! the component returns.
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! # fn app() -> impl IntoElement {
//! rect()
//!     .width(Size::px(100.))
//!     .height(Size::px(100.))
//!     .background(Color::RED)
//!     .on_press(|_| println!("Clicked!"))
//! # }
//! ```
//!
//! ## Event handlers
//!
//! Most events have a method named `on_<event>` (and global variants `on_global_<event>`).
//! See [`EventHandlersExt`](freya_core::prelude::EventHandlersExt) for the full list.
//!
//! Common ones:
//!
//! - Pointer / mouse / touch: `on_press`, `on_secondary_down`, `on_pointer_press`, `on_pointer_down`,
//!   `on_pointer_move`, `on_pointer_enter`, `on_pointer_leave`, `on_pointer_over`, `on_pointer_out`.
//! - Keyboard: `on_key_down`, `on_key_up`.
//! - Wheel: `on_wheel`.
//! - Layout: `on_sized`.
//! - Files: `on_file_drop`.
//!
//! Each handler receives an [`Event<D>`](freya_core::prelude::Event) where `D` is the payload
//! (e.g [`PointerEventData`](freya_core::prelude::PointerEventData),
//! [`KeyboardEventData`](freya_core::prelude::KeyboardEventData), etc).
//!
//! ## Propagation (bubbling)
//!
//! When an event fires on a target element it then **bubbles** up through its ancestors,
//! invoking any matching handler along the way.
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! # fn app() -> impl IntoElement {
//! rect()
//!     .on_press(|_| println!("outer"))
//!     .child(rect().on_press(|_| println!("inner")))
//! # }
//! ```
//!
//! Pressing the inner rect prints `inner` and then `outer`.
//!
//! ### Stopping propagation
//!
//! Call [`stop_propagation`](freya_core::prelude::Event::stop_propagation) on the event to
//! prevent it from continuing to bubble up to ancestors. The current handler still runs to
//! completion; only ancestor handlers for **this same event** are skipped.
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! # fn app() -> impl IntoElement {
//! rect()
//!     .on_press(|_| println!("outer"))
//!     .child(rect().on_press(|e: Event<PressEventData>| {
//!         e.stop_propagation();
//!         println!("inner only");
//!     }))
//! # }
//! ```
//!
//! Note that not every event bubbles. Move/enter/leave events, capture events, and global
//! events do not bubble: they target specific elements directly.
//!
//! ## Default behavior and `prevent_default`
//!
//! Some events have a **default behavior**: side effects that Freya runs after the handler
//! unless you opt out. The most common case is that an element-level event (e.g `on_mouse_up`)
//! also dispatches a related **global** event (e.g `on_global_pointer_press`), and that some
//! events (e.g `on_mouse_down`) get translated into the unified
//! [pointer events](freya_core::prelude::PointerEventData) (e.g `on_pointer_down`).
//!
//! Calling [`prevent_default`](freya_core::prelude::Event::prevent_default) cancels those
//! follow-up events for this dispatch. It is **not** the same as `stop_propagation`:
//!
//! - `stop_propagation`: stops **this same event** from bubbling to ancestors.
//! - `prevent_default`: cancels **other related events** that would otherwise fire as a
//!   consequence of this one.
//!
//! Each event declares which other events it can cancel. For example a `on_mouse_up` handler
//! that calls `prevent_default` will additionally cancel the `on_pointer_press` and the
//! `on_global_pointer_press` events that would have fired afterwards.
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! # fn app() -> impl IntoElement {
//! rect()
//!     .on_global_pointer_press(|_| println!("Anywhere on screen"))
//!     .child(rect().on_mouse_up(|e: Event<MouseEventData>| {
//!         // Suppresses the global handler above for this click only.
//!         e.prevent_default();
//!         println!("Click handled here");
//!     }))
//! # }
//! ```
//!
//! A typical use case is a draggable scrollbar: while dragging, the scrollbar handles the
//! pointer move events itself and calls `prevent_default` so that no other element behind the
//! cursor receives hover/move events for the duration of the drag.
//!
//! ## Event ordering and priority
//!
//! When several events would dispatch in the same frame, Freya processes them in a fixed
//! priority order:
//!
//! 1. **Capture** events (e.g `on_capture_global_pointer_press`, `on_capture_global_pointer_move`).
//! 2. **Leave** events (`on_pointer_leave`, `on_pointer_out`).
//! 3. **Enter / over** events (`on_pointer_enter`, `on_pointer_over`).
//! 4. Everything else.
//!
//! Within the same priority class, events are sorted by layer and cursor position so that
//! the topmost element under the cursor is reached first. Because of this, a handler that
//! calls `prevent_default` on a higher-priority event can naturally cancel lower-priority
//! events that would otherwise have fired in the same frame.
//!
//! Capture events are the strongest tool here: they run before anything else, and a
//! `prevent_default` on a capture handler can cancel the regular pointer events for that
//! tick. They are useful when you need to intercept input globally before any element sees
//! it (for example, dismissing an overlay on the next click anywhere).
//!
//! ## Global events
//!
//! Global events fire **once per dispatch**, regardless of which element is under the cursor
//! or focused. They are useful when you want to react to input that is not necessarily aimed
//! at your element.
//!
//! - `on_global_pointer_press`
//! - `on_global_pointer_down`
//! - `on_global_pointer_move`
//! - `on_global_key_down`
//! - `on_global_key_up`
//! - `on_global_file_hover`
//! - `on_global_file_hover_cancelled`
//!
//! Global events do **not** bubble (they are dispatched directly to every listener), so
//! `stop_propagation` has no effect on them. A non-global handler that calls
//! `prevent_default` will, however, suppress the matching global event for that dispatch.
//!
//! ## Components don't have events
//!
//! Components are just data and a `render` method. To expose a "click" or "change" hook from
//! a component, accept a callback as a field and forward it from the inner element.
//!
//! ```rust, no_run
//! # use freya::prelude::*;
//! #[derive(PartialEq)]
//! struct MyButton {
//!     on_press: EventHandler<Event<PressEventData>>,
//! }
//!
//! impl Component for MyButton {
//!     fn render(&self) -> impl IntoElement {
//!         rect()
//!             .padding(8.)
//!             .background(Color::BLUE)
//!             .on_press(self.on_press.clone())
//!             .child("Press me")
//!     }
//! }
//! ```
