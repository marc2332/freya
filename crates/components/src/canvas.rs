use crate::theme::get_theme;
use dioxus::prelude::{render, Element, Props, Scope};
use freya_elements::elements as dioxus_elements;
use freya_hooks::{CanvasTheme, CanvasThemeWith, UseCanvas};

/// [`Canvas`] component properties.
#[derive(Props, PartialEq)]
pub struct CanvasProps {
    /// Theme override.
    pub theme: Option<CanvasThemeWith>,
    /// The Canvas reference.
    pub canvas: UseCanvas,
}

/// Draw anything inside of this canvas.
///
/// # Props
/// See [`CanvasProps`].
///
#[allow(non_snake_case)]
pub fn Canvas(cx: Scope<CanvasProps>) -> Element {
    let CanvasTheme {
        width,
        height,
        background,
    } = get_theme!( cx, &cx.props.theme, canvas );

    render!(rect { overflow: "clip", canvas_reference: cx.props.canvas.attribute(cx), background: "{background}", width: "{width}", height: "{height}" })
}
