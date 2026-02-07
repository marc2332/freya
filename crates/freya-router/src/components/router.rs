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

pub struct Router<R: Routable + Clone>(NoArgCallback<RouterConfig<R>>);

impl<R: Routable + Clone> PartialEq for Router<R> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<R: Routable + Clone> Router<R> {
    pub fn new(init: impl Into<NoArgCallback<RouterConfig<R>>>) -> Self {
        Self(init.into())
    }
}

impl<R: Routable + Clone> Component for Router<R> {
    fn render(&self) -> impl IntoElement {
        use_hook(|| {
            provide_context(RouterContext::create::<R>(self.0.call()));
            provide_context(OutletContext::<R>::new());
        });

        Outlet::<R>::new()
    }
}

pub fn use_share_router(router: impl FnOnce() -> RouterContext) {
    use_provide_context(router);
}
