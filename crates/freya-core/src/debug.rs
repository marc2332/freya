#[cfg(debug_assertions)]
use crate::prelude::Color;
#[cfg(debug_assertions)]
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
    #[cfg(debug_assertions)]
    fn debug(self) -> Self {
        self.border(Border::new().width(2.).fill(Color::RED))
            .shadow((0., 0., 10., 5., (0, 0, 0, 0.2)))
    }

    #[cfg(not(debug_assertions))]
    fn debug(self) -> Self {
        self
    }
}
