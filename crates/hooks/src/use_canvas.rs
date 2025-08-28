use std::sync::{
    Arc,
    Mutex,
};

use dioxus_core::AttributeValue;
use dioxus_hooks::{
    use_memo,
    use_reactive,
    Dependency,
};
use dioxus_signals::{
    Memo,
    ReadableExt,
};
use freya_core::custom_attributes::{
    CanvasReference,
    CanvasRunner,
    CanvasRunnerContext,
    CustomAttributeValues,
};

/// Holds a rendering hook callback that allows to render to the Canvas.
#[derive(PartialEq, Clone)]
pub struct UseCanvas {
    runner: Memo<UseCanvasRunner>,
}

#[derive(Clone)]
pub struct UseCanvasRunner(pub Arc<Mutex<CanvasRunner>>);

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
///     let (reference, size) = use_node_signal();
///     let mut value = use_signal(|| 0);
///     let platform = use_platform();
///
///     let canvas = use_canvas(move || {
///         let curr = value();
///         platform.invalidate_drawing_area(size.peek().area);
///         platform.request_animation_frame();
///         move |ctx| {
///             // Draw using the canvas !
///             // use `curr`
///         }
///     });
///
///     rsx!(rect {
///         onclick: move |_| {
///             value += 1;
///         },
///         canvas_reference: canvas.attribute(),
///         reference,
///         width: "fill",
///         height: "fill",
///     })
/// }
/// ```
pub fn use_canvas<T: FnMut(&mut CanvasRunnerContext) + Send + 'static>(
    mut renderer_cb: impl FnMut() -> T + 'static,
) -> UseCanvas {
    let runner = use_memo(move || UseCanvasRunner(Arc::new(Mutex::new(renderer_cb()))));

    UseCanvas { runner }
}

/// Register a rendering hook to gain access to the Canvas.
/// Reactivity managed with manual dependencies.
///
/// ## Usage
/// ```rust,no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let (reference, size) = use_node_signal();
///     let mut value = use_signal(|| 0);
///     let platform = use_platform();
///
///     let canvas = use_canvas_with_deps(&value(), move |curr| {
///         platform.invalidate_drawing_area(size.peek().area);
///         platform.request_animation_frame();
///         move |ctx| {
///             // Draw using the canvas !
///             // use `curr`
///         }
///     });
///
///     rsx!(rect {
///         onclick: move |_| {
///             value += 1;
///         },
///         canvas_reference: canvas.attribute(),
///         reference,
///         width: "fill",
///         height: "fill",
///     })
/// }
/// ```
pub fn use_canvas_with_deps<
    D: Dependency,
    T: FnMut(&mut CanvasRunnerContext) + Sync + Send + 'static,
>(
    dependencies: D,
    mut renderer_cb: impl FnMut(D::Out) -> T + 'static,
) -> UseCanvas
where
    D::Out: 'static,
{
    let runner = use_memo(use_reactive(dependencies, move |dependencies| {
        UseCanvasRunner(Arc::new(Mutex::new(renderer_cb(dependencies))))
    }));

    UseCanvas { runner }
}
