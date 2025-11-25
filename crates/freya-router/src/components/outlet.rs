use std::marker::PhantomData;

use freya_core::prelude::*;

use crate::prelude::{
    outlet::OutletContext,
    *,
};

struct Outlet<R>(PhantomData<R>);

impl<R> PartialEq for Outlet<R> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<R: Routable> Render for Outlet<R> {
    fn render(&self) -> impl IntoElement {
        OutletContext::<R>::render()
    }
}

pub fn outlet<R: Routable + Clone>() -> Element {
    Outlet::<R>(PhantomData).into()
}
