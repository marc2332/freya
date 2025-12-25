use freya_core::prelude::{
    IntoElement,
    Render,
    provide_context,
    try_consume_context,
    use_hook,
};

/// Components that allow the macro to add child routers. This component provides a context
/// to the child router that maps child routes to root routes and vice versa.
use crate::prelude::Routable;

/// Maps a child route into the root router and vice versa
// NOTE: Currently child routers only support simple static prefixes, but this
// API could be expanded to support dynamic prefixes as well
pub(crate) struct ChildRouteMapping<R> {
    format_route_as_root_route: fn(R) -> String,
    parse_route_from_root_route: fn(&str) -> Option<R>,
}

impl<R: Routable> ChildRouteMapping<R> {
    pub(crate) fn format_route_as_root_route(&self, route: R) -> String {
        (self.format_route_as_root_route)(route)
    }

    pub(crate) fn parse_route_from_root_route(&self, route: &str) -> Option<R> {
        (self.parse_route_from_root_route)(route)
    }
}

/// Get the formatter that handles adding and stripping the prefix from a child route
pub(crate) fn consume_child_route_mapping<R: Routable>() -> Option<ChildRouteMapping<R>> {
    try_consume_context()
}

impl<R> Clone for ChildRouteMapping<R> {
    fn clone(&self) -> Self {
        Self {
            format_route_as_root_route: self.format_route_as_root_route,
            parse_route_from_root_route: self.parse_route_from_root_route,
        }
    }
}

pub struct ChildRouter<R: Routable> {
    /// The child route to render
    route: R,
    /// Take a parent route and return a child route or none if the route is not part of the child
    parse_route_from_root_route: fn(&str) -> Option<R>,
    /// Take a child route and return a parent route
    format_route_as_root_route: fn(R) -> String,
}

impl<R: Routable> PartialEq for ChildRouter<R> {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl<R: Routable> Render for ChildRouter<R> {
    fn render(&self) -> impl IntoElement {
        use_hook(|| {
            provide_context(ChildRouteMapping {
                format_route_as_root_route: self.format_route_as_root_route,
                parse_route_from_root_route: self.parse_route_from_root_route,
            })
        });
        self.route.render(0)
    }
}
