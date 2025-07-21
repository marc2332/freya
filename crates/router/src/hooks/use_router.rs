use crate::prelude::RouterContext;

/// Acquire the router without subscribing to updates.
pub fn router() -> RouterContext {
    dioxus::prelude::consume_context()
}

/// Try to acquire the router without subscribing to updates.
pub fn try_router() -> Option<RouterContext> {
    dioxus::prelude::try_consume_context()
}
