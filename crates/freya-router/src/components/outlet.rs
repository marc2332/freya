use std::marker::PhantomData;

use freya_core::prelude::*;

use crate::prelude::{
    outlet::OutletContext,
    *,
};

pub struct Outlet<R>(PhantomData<R>);

impl<R> Outlet<R> {
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
