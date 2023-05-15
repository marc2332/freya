use std::sync::Arc;

use dioxus_core::{AttributeValue, Scope, ScopeState};
use freya_common::Area;
use freya_node_state::{CanvasReference, CustomAttributeValues};
use skia_safe::{textlayout::FontCollection, Canvas};
use uuid::Uuid;

pub type RenderCallback = Box<dyn Fn(&mut Canvas, &FontCollection, Area)>;

/// Holds a rendering hook callback that allows to render to the Canvas.
pub struct UseCanvas {
    id: Uuid,
    hook_callback: Arc<RenderCallback>,
}

impl PartialEq for UseCanvas {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl UseCanvas {
    pub fn attribute<'a, T>(&self, cx: Scope<'a, T>) -> AttributeValue<'a> {
        cx.any_value(CustomAttributeValues::Canvas(CanvasReference {
            runner: self.hook_callback.clone(),
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
///         Box::new(|canvas, font_collection, area| {
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
pub fn use_canvas(cx: &ScopeState, renderer: impl FnOnce() -> RenderCallback) -> UseCanvas {
    let id = cx.use_hook(Uuid::new_v4);
    let renderer = cx.use_hook(|| Arc::new(renderer()));

    UseCanvas {
        id: *id,
        hook_callback: renderer.clone(),
    }
}
