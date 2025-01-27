//! ## Elements and events
//!
//! - [Elements](crate::elements#structs)
//! - [Events](crate::elements#functions)

pub mod _docs;

mod definitions;
pub mod events;

pub mod elements {
    pub use crate::{
        definitions::*,
        events::*,
    };
}

pub use crate::definitions::*;
