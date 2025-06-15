use crate::{
    prelude::*,
    utils::use_router_internal::use_router_internal,
};

/// A hook that provides access to information about the current routing location.
///
/// > The Routable macro will define a version of this hook with an explicit type.
///
/// # Panic
/// - When the calling component is not nested within a [`Router`] component.
///
/// # Example
/// ```rust
/// # use freya::prelude::*;
/// # use freya_router::{prelude::*};
///
/// #[derive(Clone, Routable)]
/// enum Route {
///     #[route("/")]
///     Index {},
/// }
///
/// #[component]
/// fn App() -> Element {
///     rsx!(
///         label { "App" }
///         Router::<Route> {}
///     )
/// }
///
/// #[component]
/// fn Index() -> Element {
///     let path: Route = use_route();
///     rsx!(
///         label { "Current Path: {path}" }
///     )
/// }
/// ```
#[doc(alias = "use_url")]
#[must_use]
pub fn use_route<R: Routable + Clone>() -> R {
    match use_router_internal() {
        Some(r) => r.current(),
        None => {
            panic!("`use_route` must be called in a descendant of a Router component")
        }
    }
}
