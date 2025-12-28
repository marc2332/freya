#[cfg(debug_assertions)]
use crate::prelude::Color;
use crate::prelude::{
    Border,
    StyleExt,
};

pub trait DebugExt
where
    Self: Sized,
{
    fn debug(self) -> Self;
}

impl<T: StyleExt> DebugExt for T {
    fn debug(mut self) -> Self {
        #[cfg(debug_assertions)]
        {
            self = self
                .border(Border::new().width(2.).fill(Color::RED))
                .shadow((0., 0., 10., 5., (0, 0, 0, 0.2)));
        }
        self
    }
}
