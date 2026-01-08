use freya_core::prelude::*;
use freya_router::{
    hooks::use_route,
    prelude::Routable,
};

use crate::activable_route_context::ActivableRouteContext;

#[derive(PartialEq, Clone)]
pub struct ActivableRoute<T> {
    child: Element,
    route: T,
    exact: bool,
    routes: Vec<T>,
}

impl<T> ActivableRoute<T> {
    pub fn new(route: T, child: impl Into<Element>) -> Self {
        Self {
            child: child.into(),
            route,
            exact: false,
            routes: vec![],
        }
    }

    pub fn exact(mut self, exact: bool) -> Self {
        self.exact = exact;
        self
    }

    pub fn routes(mut self, routes: Vec<T>) -> Self {
        self.routes = routes;
        self
    }
}

impl<T: PartialEq + Clone + 'static + Routable> Component for ActivableRoute<T> {
    fn render(&self) -> impl IntoElement {
        let current_route = use_route::<T>();

        let is_descendent_route_active = current_route.is_child_of(&self.route);
        let is_descendent_routes_active = self
            .routes
            .iter()
            .any(|route| current_route.is_child_of(route));
        let is_descendent_active =
            !self.exact && (is_descendent_route_active || is_descendent_routes_active);

        let is_exact_active = current_route == self.route || self.routes.contains(&current_route);

        let is_active = is_descendent_active || is_exact_active;

        let mut ctx = use_provide_context::<ActivableRouteContext>(|| {
            ActivableRouteContext(State::create(is_active))
        });

        if *ctx.0.peek() != is_active {
            *ctx.0.write() = is_active;
        }

        self.child.clone()
    }
}
