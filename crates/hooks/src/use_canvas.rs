use std::sync::Arc;

use dioxus_core::AttributeValue;
use dioxus_signals::{use_memo_with_dependencies, Dependency, Readable};
use freya_node_state::{CanvasReference, CanvasRunner, CustomAttributeValues};

/// Holds a rendering hook callback that allows to render to the Canvas.
#[derive(PartialEq, Clone)]
pub struct UseCanvas {
    runner: UseCanvasRunner,
}

#[derive(Clone)]
pub struct UseCanvasRunner(pub Arc<Box<CanvasRunner>>);

impl PartialEq for UseCanvasRunner {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl UseCanvas {
    pub fn attribute(&self) -> AttributeValue {
        AttributeValue::any_value(CustomAttributeValues::Canvas(CanvasReference {
            runner: self.runner.0.clone(),
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
///     rsx!(
///         Canvas {
///             canvas: canvas
///         }
///     )
/// }
/// ```
pub fn use_canvas<D: Dependency>(
    dependencies: D,
    renderer_cb: impl Fn(D::Out) -> Box<CanvasRunner> + 'static,
) -> UseCanvas
where
    D::Out: 'static,
{
    let runner = use_memo_with_dependencies(dependencies, move |dependencies| {
        UseCanvasRunner(Arc::new(renderer_cb(dependencies)))
    });

    UseCanvas {
        runner: runner.peek().clone(),
    }
}
