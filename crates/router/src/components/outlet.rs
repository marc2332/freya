use dioxus_lib::prelude::*;

use crate::prelude::{
    outlet::OutletContext,
    *,
};

/// An outlet for the current content.
///
/// The [`Outlet`] is aware of how many [`Outlet`]s it is nested within. It will render the content
/// of the active route that is __exactly as deep__.
///
/// # Example
/// ```rust
/// # use dioxus::prelude::*;
/// # use freya_router::prelude::*;
/// #[derive(Clone, Routable)]
/// #[rustfmt::skip]
/// enum Route {
///     #[nest("/wrap")]
///         #[layout(Wrapper)] // Every layout component must have one Outlet
///             #[route("/")]
///             Child {},
///         #[end_layout]
///     #[end_nest]
///     #[route("/")]
///     Index {},
/// }
///
/// #[component]
/// fn Index() -> Element {
///     rsx!(
///         label {
///             "Index"
///         }
///     )
/// }
///
/// #[component]
/// fn Wrapper() -> Element {
///     rsx!(
///         label { "App" }
///         Outlet::<Route> {} // The content of child routes will be rendered here
///     )
/// }
///
/// #[component]
/// fn Child() -> Element {
///     rsx!(
///         label {
///             "Child"
///         }
///     )
/// }
/// ```
pub fn Outlet<R: Routable + Clone>() -> Element {
    OutletContext::<R>::render()
}
