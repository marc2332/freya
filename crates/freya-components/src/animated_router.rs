use std::marker::PhantomData;

use freya_core::prelude::*;
use freya_router::prelude::{
    Routable,
    use_route,
};

#[derive(Clone)]
pub enum AnimatedRouterContext<R: Routable + PartialEq> {
    /// Transition from one route to another.
    FromTo(R, R),
    /// Settled in a route.
    In(R),
}

impl<R: Routable + PartialEq> AnimatedRouterContext<R> {
    /// Get the current destination route.
    pub fn target_route(&self) -> &R {
        match self {
            Self::FromTo(_, to) => to,
            Self::In(to) => to,
        }
    }

    /// Update the destination route.
    pub fn set_target_route(&mut self, to: R) {
        match self {
            Self::FromTo(old_from, old_to) => {
                *old_from = old_to.clone();
                *old_to = to
            }
            Self::In(old_to) => *self = Self::FromTo(old_to.clone(), to),
        }
    }

    /// After the transition animation has finished, make the outlet only render the destination route.
    pub fn settle(&mut self) {
        if let Self::FromTo(_, to) = self {
            *self = Self::In(to.clone())
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct AnimatedRouter<R> {
    content: Element,
    _phantom: PhantomData<R>,
}

impl<R> AnimatedRouter<R> {
    pub fn new(children: impl Into<Element>) -> Self {
        Self {
            content: children.into(),
            _phantom: PhantomData::<R>,
        }
    }
}

/// Provide a mechanism for [freya_router::prelude::Outlet] to animate between route changes.
///
/// See the `animated_router.rs` example to see how to use it.
impl<R: Routable + 'static + PartialEq> Component for AnimatedRouter<R> {
    fn render(&self) -> impl IntoElement {
        let route = use_route::<R>();
        let mut prev_route = use_state(|| AnimatedRouterContext::In(route.clone()));
        use_provide_context(move || prev_route);

        if prev_route.peek().target_route() != &route {
            prev_route.write().set_target_route(route);
        }

        self.content.clone()
    }
}

/// Shortcut to get access to the [AnimatedRouterContext].
pub fn use_animated_router<Route: Routable + PartialEq>() -> State<AnimatedRouterContext<Route>> {
    use_consume()
}
