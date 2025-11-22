use freya_core::prelude::*;

use crate::{
    components::outlet,
    routable::Routable,
    router_cfg::RouterConfig,
};

/// A component that renders the current route.
pub fn router<R: Routable + Clone>(init: impl FnOnce() -> RouterConfig<R>) -> impl IntoElement {
    use crate::prelude::{
        RouterContext,
        outlet::OutletContext,
    };

    use_hook(|| {
        provide_context(RouterContext::new::<R>(init()));
        provide_context(OutletContext::<R>::new());
    });

    outlet::<R>()
}
