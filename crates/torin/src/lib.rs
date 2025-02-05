#![warn(clippy::pedantic, clippy::perf)]
#![allow(
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
    clippy::doc_markdown,
    clippy::cast_precision_loss,
    clippy::too_many_lines,
    clippy::option_option
)]

pub mod custom_measurer;
pub mod dom_adapter;
pub mod geometry;
mod measure;
pub mod node;
pub mod scaled;
pub mod sendanymap;
pub mod torin;
pub mod values;

pub use values::*;

pub mod prelude {
    pub use crate::{
        custom_measurer::*,
        dom_adapter::*,
        gaps::*,
        geometry::*,
        node::*,
        scaled::*,
        sendanymap::*,
        torin::*,
        values::prelude::*,
    };
}

pub mod test_utils;
