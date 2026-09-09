use freya_core::prelude::{
    Element,
    provide_context,
    try_consume_context,
    try_consume_own_context,
    use_hook,
};

use crate::{
    routable::Routable,
    utils::use_router_internal::use_router_internal,
};

/// A context that manages nested routing levels for outlet components.
///
/// The outlet context keeps track of the current nesting level of routes and helps
/// manage the hierarchical structure of nested routes in the application.
///
/// # Type Parameters
///
/// * `R` - The routable type that implements the routing logic
#[derive(Clone, Default)]
pub struct OutletContext<R> {
    current_level: usize,
    _marker: std::marker::PhantomData<R>,
}

impl<R> OutletContext<R> {
    /// Creates a new outlet context starting at level 0
    pub fn new() -> Self {
        Self {
            current_level: 0,
            _marker: std::marker::PhantomData,
        }
    }

    /// Creates a new outlet context for the next nesting level
    pub fn next(&self) -> Self {
        Self {
            current_level: self.current_level + 1,
            _marker: std::marker::PhantomData,
        }
    }

    /// Creates the outlet context of the previous nesting level
    pub fn previous(&self) -> Self {
        Self {
            current_level: self.current_level.saturating_sub(1),
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns the current nesting level of this outlet
    pub fn level(&self) -> usize {
        self.current_level
    }

    pub(crate) fn render() -> Element
    where
        R: Routable + Clone,
    {
        let router = use_router_internal().expect("Outlet must be inside of a router");
        let outlet: OutletContext<R> = use_outlet_context();
        router.current::<R>().render(outlet.level())
    }
}

pub fn use_outlet_context<R: Clone + 'static>() -> OutletContext<R> {
    use_hook(|| {
        if let Some(next) = try_consume_own_context::<OutletContext<R>>() {
            return next.previous();
        }
        let outlet: OutletContext<R> = try_consume_context().unwrap_or_else(OutletContext::new);
        provide_context(outlet.next());
        outlet
    })
}
