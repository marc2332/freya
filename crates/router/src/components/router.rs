use dioxus_lib::prelude::*;

use crate::{
    prelude::Outlet,
    routable::Routable,
    router_cfg::RouterConfig,
};

/// The props for [`Router`].
#[derive(Props, Clone, Copy)]
pub struct RouterProps<R: Routable> {
    #[props(default, into)]
    config: Callback<(), RouterConfig<R>>,
}

impl<R: Routable> Default for RouterProps<R> {
    fn default() -> Self {
        Self {
            config: Callback::new(|_| RouterConfig::default()),
        }
    }
}

impl<R: Routable> PartialEq for RouterProps<R> {
    fn eq(&self, _: &Self) -> bool {
        // prevent the router from re-rendering when the initial url or config changes
        true
    }
}

/// A component that renders the current route.
pub fn Router<R: Routable + Clone>(props: RouterProps<R>) -> Element {
    use crate::prelude::{
        outlet::OutletContext,
        RouterContext,
    };

    use_hook(|| {
        provide_context(RouterContext::new::<R>(props.config.call(())));
        provide_context(OutletContext::<R>::new());
    });

    rsx! { Outlet::<R> {} }
}
