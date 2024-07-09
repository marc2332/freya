use std::sync::Arc;

use dioxus_core::AttributeValue;
use dioxus_hooks::{use_memo, use_reactive, Dependency};
use dioxus_signals::{Memo, Readable};
use freya_node_state::{CanvasReference, CanvasRunner, CustomAttributeValues};

/// Holds a rendering hook callback that allows to render to the Canvas.
#[derive(PartialEq, Clone)]
pub struct UseCanvas {
    runner: Memo<UseCanvasRunner>,
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
            runner: self.runner.read().0.clone(),
        }))
    }
}

/// Register a rendering hook to gain access to the Canvas.
/// Reactivity managed through signals.
///
/// ## Usage
/// ```rust,no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let value = use_signal(|| 0);
///
///     let canvas = use_canvas(move || {
///         let curr = value();
///         Box::new(move |canvas, font_collection, area, scale_factor| {
///             // Draw using the canvas !
///             // use `curr`
///         })
///     });
///
///     rsx!(Canvas { canvas })
/// }
/// ```
pub fn use_canvas(renderer_cb: impl Fn() -> Box<CanvasRunner> + 'static) -> UseCanvas {
    let runner = use_memo(move || UseCanvasRunner(Arc::new(renderer_cb())));

    UseCanvas { runner }
}

/// Register a rendering hook to gain access to the Canvas.
/// Reactivity managed with manual dependencies.
///
/// ## Usage
/// ```rust,no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let value = use_signal(|| 0);
///
///     let canvas = use_canvas_with_deps(&value(), |curr| {
///         Box::new(move |canvas, font_collection, area, scale_factor| {
///             // Draw using the canvas !
///             // use `curr`
///         })
///     });
///
///     rsx!(Canvas { canvas })
/// }
/// ```
pub fn use_canvas_with_deps<D: Dependency>(
    dependencies: D,
    renderer_cb: impl Fn(D::Out) -> Box<CanvasRunner> + 'static,
) -> UseCanvas
where
    D::Out: 'static,
{
    let runner = use_memo(use_reactive(dependencies, move |dependencies| {
        UseCanvasRunner(Arc::new(renderer_cb(dependencies)))
    }));

    UseCanvas { runner }
}
