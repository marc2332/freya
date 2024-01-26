use std::sync::Arc;

use dioxus_core::{AttributeValue, Scope, ScopeState};
use dioxus_hooks::{use_memo, UseFutureDep};
use freya_node_state::{CanvasReference, CanvasRunner, CustomAttributeValues};
use uuid::Uuid;

/// Holds a rendering hook callback that allows to render to the Canvas.
pub struct UseCanvas {
    id: Uuid,
    runner: Arc<Box<CanvasRunner>>,
}

impl PartialEq for UseCanvas {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl UseCanvas {
    pub fn attribute<'a, T>(&self, cx: Scope<'a, T>) -> AttributeValue<'a> {
        cx.any_value(CustomAttributeValues::Canvas(CanvasReference {
            id: self.id,
            runner: self.runner.clone(),
        }))
    }
}

/// Register a rendering hook to gain access to the Canvas.
///
/// ## Usage
/// ```rust,no_run
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
    let (id, runner) = use_memo(cx, dependencies, |dependencies| {
        (Uuid::new_v4(), Arc::new(renderer_cb(dependencies)))
    });

    UseCanvas {
        id: *id,
        runner: runner.clone(),
    }
}
