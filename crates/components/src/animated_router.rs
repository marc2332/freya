use dioxus::prelude::*;
use dioxus_router::prelude::{
    use_route,
    Routable,
};

#[derive(Clone)]
pub enum AnimatedRouterContext<R: Routable + PartialEq> {
    FromTo(R, R),
    In(R),
}

impl<R: Routable + PartialEq> AnimatedRouterContext<R> {
    pub fn target_route(&self) -> &R {
        match self {
            Self::FromTo(_, to) => to,
            Self::In(to) => to,
        }
    }

    pub fn set_target_route(&mut self, to: R) {
        match self {
            Self::FromTo(old_from, old_to) => {
                *old_from = old_to.clone();
                *old_to = to
            }
            Self::In(old_to) => *self = Self::FromTo(old_to.clone(), to),
        }
    }

    pub fn settle(&mut self) {
        if let Self::FromTo(_, to) = self {
            *self = Self::In(to.clone())
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AnimatedRouterProps {
    children: Element,
}

#[allow(non_snake_case)]
pub fn AnimatedRouter<R: Routable + PartialEq + Clone>(
    AnimatedRouterProps { children }: AnimatedRouterProps,
) -> Element {
    let route = use_route::<R>();
    let mut prev_route = use_signal(|| AnimatedRouterContext::In(route.clone()));
    use_context_provider(move || prev_route);

    if prev_route.peek().target_route() != &route {
        prev_route.write().set_target_route(route);
    }

    rsx!({ children })
}

pub fn use_animated_router<Route: Routable + PartialEq>() -> Signal<AnimatedRouterContext<Route>> {
    use_context()
}
