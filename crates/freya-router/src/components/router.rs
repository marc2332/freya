use freya_core::prelude::*;

use crate::{
    prelude::{
        Outlet,
        OutletContext,
        RouterContext,
    },
    routable::Routable,
    router_cfg::RouterConfig,
};

/// Provides routing state and renders the active route.
///
/// Mount one router at the root of a window. It creates a [`RouterContext`] and a
/// root [`Outlet`], which renders the component associated with the current route.
/// Layout components can add further outlets for nested routes.
///
/// See the [basic router example](https://github.com/marc2332/freya/blob/main/examples/feature_router.rs).
pub struct Router<R: Routable + Clone>(NoArgCallback<RouterConfig<R>>);

impl<R: Routable + Clone> PartialEq for Router<R> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<R: Routable + Clone> Router<R> {
    /// Creates a router using the configuration returned by `init`.
    ///
    /// Use [`RouterConfig::with_initial_path`] to select a route other than `/`
    /// when the router starts.
    pub fn new(init: impl Into<NoArgCallback<RouterConfig<R>>>) -> Self {
        Self(init.into())
    }
}

impl<R: Routable + Clone> Component for Router<R> {
    fn render(&self) -> impl IntoElement {
        use_hook(|| {
            // Preserve routing state when hot reload resets hook values.
            if try_consume_own_context::<RouterContext>().is_none() {
                provide_context(RouterContext::create::<R>(self.0.call()));
            }
            if try_consume_own_context::<OutletContext<R>>().is_none() {
                provide_context(OutletContext::<R>::new());
            }
        });

        Outlet::<R>::new()
    }
}

/// Provides an existing router context to this component subtree.
///
/// Use this with a [`RouterContext::create_global`] context to share navigation
/// state between windows. Call it at the window's root component.
///
/// See the [multi-window router example](https://github.com/marc2332/freya/blob/main/examples/feature_multi_window_router.rs).
pub fn use_share_router(router: impl FnOnce() -> RouterContext) {
    use_provide_context(router);
}
