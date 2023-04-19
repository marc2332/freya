use dioxus::prelude::{render, Element, Props, Scope};
use freya_elements::elements as dioxus_elements;
use freya_hooks::UseCanvas;

#[derive(Props, PartialEq)]
pub struct CanvasProps {
    #[props(default = "300".to_string(), into)]
    width: String,

    #[props(default = "150".to_string(), into)]
    height: String,

    #[props(default = "white".to_string(), into)]
    background: String,

    canvas: UseCanvas,
}

#[allow(non_snake_case)]
pub fn Canvas(cx: Scope<CanvasProps>) -> Element {
    render!(container {
        canvas_reference: cx.props.canvas.attribute(cx),
        background: "{cx.props.background}",
        width: "{cx.props.width}",
        height: "{cx.props.height}",
    })
}
