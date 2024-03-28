pub mod dom;
pub mod dom_adapter;
mod mutations_writer;
mod paragraph_utils;

pub mod prelude {
    pub use crate::dom::*;
    pub use crate::dom_adapter::*;
}
