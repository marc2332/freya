use crate::{
    prelude::*,
    utils::use_router_internal::use_router_internal,
};

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
