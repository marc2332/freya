use dioxus::prelude::*;
use dioxus_router::{
    hooks::use_route,
    prelude::Routable,
};
use freya_hooks::ActivableRouteContext;

/// Provide a context to the inner components so they can know whether the passed route is the current router in the Router or not.
#[allow(non_snake_case)]
#[component]
pub fn ActivableRoute<T: Clone + PartialEq + Routable + 'static>(
    children: Element,
    route: T,
    #[props(default = false)] exact: bool,
) -> Element {
    let current_route = use_route::<T>();
    let is_active = (!exact && current_route.is_child_of(&route)) || current_route == route;
    let mut ctx = use_context_provider::<ActivableRouteContext>(|| {
        ActivableRouteContext(Signal::new(is_active))
    });

    if *ctx.0.peek() != is_active {
        *ctx.0.write() = is_active;
    }

    rsx!({ children })
}
