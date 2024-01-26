pub mod dom;
pub mod dom_adapter;
mod mutations_writer;

pub mod prelude {
    pub use crate::dom::*;
    pub use crate::dom_adapter::*;
}
