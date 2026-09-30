use std::marker::PhantomData;

use freya_core::prelude::*;

use crate::prelude::{
    outlet::OutletContext,
    *,
};

/// Renders the active route at this nesting level.
///
/// [`Router`] includes a root outlet automatically. Add `Outlet::<Route>::new()`
/// to a layout component to render a nested route. An outlet must be a descendant
/// of a router.
///
/// See the [nested routes example](https://github.com/marc2332/freya/blob/main/examples/feature_router_complex.rs).
pub struct Outlet<R>(PhantomData<R>);

impl<R> Default for Outlet<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> Outlet<R> {
    /// Creates an outlet for routes of type `R`.
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<R> PartialEq for Outlet<R> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<R: Routable> Component for Outlet<R> {
    fn render(&self) -> impl IntoElement {
        OutletContext::<R>::render()
    }
}
