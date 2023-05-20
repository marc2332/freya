use std::sync::{Arc, Mutex};

use dioxus_core::{AttributeValue, Scope, ScopeState};
use dioxus_hooks::{to_owned, use_effect, UseFutureDep};
use freya_node_state::{CanvasReference, CanvasRunner, CustomAttributeValues};
use uuid::Uuid;

/// Holds a rendering hook callback that allows to render to the Canvas.
pub struct UseCanvas {
    id: Uuid,
    hook_callback: Arc<Mutex<Box<CanvasRunner>>>,
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
///     let canvas = use_canvas(cx, (), |_| {
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
pub fn use_canvas<D>(
    cx: &ScopeState,
    dependencies: D,
    renderer_cb: impl Fn(D::Out) -> Box<CanvasRunner>,
) -> UseCanvas
where
    D: UseFutureDep,
{
    let id = cx.use_hook(Uuid::new_v4);
    let renderer = cx.use_hook(|| Arc::new(Mutex::new(renderer_cb(dependencies.out()))));

    use_effect(cx, dependencies.clone(), {
        to_owned![renderer];
        move |_| {
            *renderer.lock().unwrap() = renderer_cb(dependencies.out());
            async move {}
        }
    });

    UseCanvas {
        id: *id,
        hook_callback: renderer.clone(),
    }
}
