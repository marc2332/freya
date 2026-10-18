use crate::{
    prelude::*,
    utils::use_router_internal::use_router_internal,
};

/// Returns the route currently rendered by the nearest [`Router`](crate::components::Router).
///
/// The component rerenders when navigation changes. Call this hook at the top
/// level of a component render method. It panics when called outside a router.
///
/// See the [complex router example](https://github.com/marc2332/freya/blob/main/examples/feature_router_complex.rs).
#[must_use]
#[track_caller]
pub fn use_route<R: Routable + Clone>() -> R {
    match use_router_internal() {
        Some(r) => r.current(),
        None => {
            panic!("`use_route` must be called in a descendant of a Router component")
        }
    }
}
