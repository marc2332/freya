use std::path::PathBuf;

use freya_core::events::PlatformEvent;
use freya_elements::events::{
    Code,
    Force,
    Key,
    Modifiers,
    MouseButton,
    TouchPhase,
};
use torin::prelude::CursorPoint;
use typed_builder::TypedBuilder;

pub type TestEvent = PlatformEvent;
