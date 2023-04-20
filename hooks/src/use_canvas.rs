use std::sync::Arc;

use dioxus_core::{AttributeValue, Scope, ScopeState};
use freya_common::Area;
use freya_node_state::{CanvasReference, CustomAttributeValues};
use skia_safe::Canvas;
use uuid::Uuid;

/// Holds the rendering hook ID.
pub struct UseCanvas {
    id: Uuid,
    renderer: Arc<Box<dyn Fn(&mut Canvas, Area) -> ()>>,
}

impl PartialEq for UseCanvas {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl UseCanvas {
    pub fn attribute<'a, T>(&self, cx: Scope<'a, T>) -> AttributeValue<'a> {
        cx.any_value(CustomAttributeValues::Canvas(CanvasReference {
            runner: self.renderer.clone(),
        }))
    }
}

/// Register a rendering hook to gain access to the Canvas.
///
/// ## Usage
/// ```rust
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///     let canvas = use_canvas(cx, || {
///         Box::new(|canvas, area| {
///             // Draw using the canvas !
///         })
///     });
///
///     render!(
///         Canvas {
///             canvas: canvas
///         }
///     )
/// }
/// ```
pub fn use_canvas(
    cx: &ScopeState,
    renderer: impl FnOnce() -> Box<dyn Fn(&mut Canvas, Area) -> ()>,
) -> UseCanvas {
    let id = cx.use_hook(Uuid::new_v4);
    let renderer = cx.use_hook(|| Arc::new(renderer()));

    UseCanvas {
        id: id.clone(),
        renderer: renderer.clone(),
    }
}
