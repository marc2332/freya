use dioxus::prelude::*;
use dioxus_router::{
    hooks::use_route,
    prelude::Routable,
};
use freya_hooks::ActivableRouteContext;

/// Sometimes you might want to know if a route is selected so you can style a specific UI element in a different way,
/// like a button with a different color.
/// To avoid cluttering your components with router-specific code you might instead want to wrap your component in an `ActivableRoute`
/// and inside your component call `use_activable_route`.
///
/// This way, your component and all its desdendants will just know whether a route is activated or not, but not which one.
///
/// ```rs
/// Link {
///     to: Route::Home, // Direction route
///     ActivableRoute {
///         route: Route::Home, // Activation route
///         SidebarItem {
///             // `SidebarItem` will now appear "activated" when the route is `Route::Home`
///             // `ActivableRoute` is letting it know whether `Route::Home` is enabled
///             // or not, without the need to add router-specific logic in `SidebarItem`.
///             label {
///                 "Go to Hey ! 👋"
///             }
///         },
///     }
/// }
/// ```
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
    let is_descendent_active =
        !exact && (is_descendent_route_active || is_descendent_routes_active);

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
