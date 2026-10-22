//! # Extending Components
//!
//! A custom [`Component`](freya_core::prelude::Component) usually wraps a built-in
//! element like [`rect()`](freya_core::prelude::rect). By default, callers cannot
//! tweak that inner element's layout, accessibility, styling, etc.
//!
//! The most common case is accepting children via [`ChildrenExt`], so callers can
//! use `.child(..)` / `.children(..)` on your component. To expose more builders,
//! Freya provides data types like [`LayoutData`] or [`AccessibilityData`], and
//! matching extension traits like [`LayoutExt`] or [`AccessibilityExt`].
//!
//! Store the data on your component, implement the matching trait, and forward
//! the data to the inner element at render time. Some builders require additional
//! traits that opt your component into specific capabilities.
//!
//! For example, the built-in [`Card`](crate::components::Card) component uses
//! [`LayoutData`] + [`LayoutExt`] and [`AccessibilityData`] + [`AccessibilityExt`]
//! to expose its inner element's layout and accessibility settings.
//!
//! For example, [`LayoutData`] stores layout settings, while [`LayoutExt`] gives
//! access to that data. To expose builders like `.width(...)` and `.padding(...)`,
//! also implement [`ContainerExt`]. This automatically provides [`ContainerSizeExt`],
//! [`ContainerPositionExt`], and [`ContainerConstraintsExt`], which can instead be
//! implemented individually if you do not need padding. Implement
//! [`ContainerWithContentExt`] separately for child layout methods like
//! `.spacing(...)` and `.center()`.
//!
//! ## Example: a `Panel` with layout builders
//!
//! ```rust
//! # use freya::prelude::*;
//! #[derive(Default, PartialEq)]
//! pub struct Panel {
//!     layout: LayoutData,
//!     elements: Vec<Element>,
//! }
//!
//! impl LayoutExt for Panel {
//!     fn layout_mut(&mut self) -> &mut LayoutData {
//!         &mut self.layout
//!     }
//! }
//!
//! impl ContainerExt for Panel {}
//!
//! impl ContainerWithContentExt for Panel {}
//!
//! impl ChildrenExt for Panel {
//!     fn children_mut(&mut self) -> &mut Vec<Element> {
//!         &mut self.elements
//!     }
//! }
//!
//! impl Component for Panel {
//!     fn render(&self) -> impl IntoElement {
//!         rect()
//!             .layout(self.layout.clone())
//!             .children(self.elements.clone())
//!     }
//! }
//!
//! fn app() -> impl IntoElement {
//!     Panel::default()
//!         .width(Size::percent(75.))
//!         .padding(8.)
//!         .spacing(4.)
//!         .center()
//!         .child("Hello, World!")
//! }
//! ```
//!
