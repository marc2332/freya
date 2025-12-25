//! Collection of components extensions or other APIs to bring Material Design style to your apps.

pub mod button;
pub mod ripple;

pub mod prelude {
    pub use crate::{
        button::*,
        ripple::*,
    };
}
