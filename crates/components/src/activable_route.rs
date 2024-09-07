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
    #[props(default = Vec::new())] routes: Vec<T>,
    #[props(default = false)] exact: bool,
) -> Element {
    let current_route = use_route::<T>();

    let is_descendent_route_active = current_route.is_child_of(&route);
    let is_descendent_routes_active = routes.iter().any(|route| current_route.is_child_of(route));
    let is_descendent_active = !exact && is_descendent_route_active && is_descendent_routes_active;

    let is_exact_active = current_route == route || routes.contains(&current_route);

    let is_active = is_descendent_active || is_exact_active;

    let mut ctx = use_context_provider::<ActivableRouteContext>(|| {
        ActivableRouteContext(Signal::new(is_active))
    });

    if *ctx.0.peek() != is_active {
        *ctx.0.write() = is_active;
    }

    rsx!({ children })
}
