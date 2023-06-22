use dioxus::prelude::{render, Element, Props, Scope};
use freya_elements::elements as dioxus_elements;
use freya_hooks::UseCanvas;

/// [`Canvas`] component properties.
#[derive(Props, PartialEq)]
pub struct CanvasProps {
    /// Width of the canvas.
    #[props(default = "300".to_string(), into)]
    width: String,
    /// Height of the canvas.
    #[props(default = "150".to_string(), into)]
    height: String,
    /// Color of the canvas.
    #[props(default = "white".to_string(), into)]
    background: String,
    /// The Canvas reference.
    canvas: UseCanvas,
}

/// Draw anything inside of this canvas.
///
/// # Props
/// See [`CanvasProps`].
///
#[allow(non_snake_case)]
pub fn Canvas(cx: Scope<CanvasProps>) -> Element {
    render!(rect {
        overflow: "clip",
        canvas_reference: cx.props.canvas.attribute(cx),
        background: "{cx.props.background}",
        width: "{cx.props.width}",
        height: "{cx.props.height}",
    })
}
