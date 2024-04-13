use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;

use freya_hooks::{use_applied_theme, CanvasTheme, CanvasThemeWith, UseCanvas};

/// Properties for the [`Canvas`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CanvasProps {
    /// Theme override.
    pub theme: Option<CanvasThemeWith>,
    /// The Canvas reference.
    pub canvas: UseCanvas,
}

/// Draw anything inside of this canvas.
#[allow(non_snake_case)]
pub fn Canvas(props: CanvasProps) -> Element {
    let CanvasTheme {
        width,
        height,
        background,
    } = use_applied_theme!(&props.theme, canvas);

    rsx!(rect {
        overflow: "clip",
        canvas_reference: props.canvas.attribute(),
        background: "{background}",
        width: "{width}",
        height: "{height}"
    })
}
