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
pub mod geometry;
pub mod measure;
pub mod node;
pub mod scaled;
pub mod torin;
pub mod tree_adapter;
pub mod values;

pub use values::*;

pub mod prelude {
    pub use crate::{
        custom_measurer::*,
        gaps::*,
        geometry::*,
        measure::*,
        node::*,
        scaled::*,
        torin::*,
        tree_adapter::*,
        values::prelude::*,
    };
}

pub mod test_utils;
