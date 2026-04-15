//! # Extending Components
//!
//! A custom [`Component`](freya_core::prelude::Component) usually wraps a built-in
//! element like `rect()`. By default, callers cannot tweak that inner element's
//! layout, accessibility, styling, etc.
//!
//! The most common case is accepting children via [`ChildrenExt`](freya_core::elements::extensions::ChildrenExt),
//! so callers can use `.child(..)` / `.children(..)` on your component. Going beyond
//! that is not super common, but when you want your component to feel like a
//! built-in element, Freya provides **Data** types like [`LayoutData`](freya_core::data::LayoutData)
//! or [`AccessibilityData`](freya_core::data::AccessibilityData), and their matching
//! **extension traits** like [`LayoutExt`](freya_core::elements::extensions::LayoutExt)
//! or [`AccessibilityExt`](freya_core::elements::extensions::AccessibilityExt).
//!
//! Store the Data on your component, implement the trait, and forward it to the
//! inner element at render time. The trait gives your component the same builder
//! methods that built-in elements expose, for free.
//!
//! The built-in [`Card`](crate::components::Card) component already does this with
//! `LayoutData` + `LayoutExt`, `AccessibilityData` + `AccessibilityExt`, and more.
//!
//! ## Example: a `Panel` with `LayoutData`
//!
//! ```rust, ignore
//! # use freya::prelude::*;
//! #[derive(Clone, PartialEq)]
//! pub struct Panel {
//!     layout: LayoutData,
//!     elements: Vec<Element>,
//! }
//!
//! impl LayoutExt for Panel {
//!     fn get_layout(&mut self) -> &mut LayoutData {
//!         &mut self.layout
//!     }
//! }
//!
//! impl ChildrenExt for Panel {
//!     fn get_children(&mut self) -> &mut Vec<Element> {
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
//! ```
//!
//! Callers can now use any layout builder method directly on `Panel`:
//!
//! ```rust, ignore
//! # use freya::prelude::*;
//! Panel::new()
//!     .width(Size::percent(75.))
//!     .padding(8.)
//!     .child("Hello, World!")
//! ```
//!
//! ## Pattern
//!
//! 1. Add a `*Data` field to your component.
//! 2. Implement the matching `*Ext` trait returning `&mut` to that field.
//! 3. Forward the Data into the inner element in `render`.
//!
//! Mix as many as you need: `LayoutData` + `LayoutExt`, `AccessibilityData` +
//! `AccessibilityExt`, `Vec<Element>` + [`ChildrenExt`](freya_core::elements::extensions::ChildrenExt),
//! and so on.
