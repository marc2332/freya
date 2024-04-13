use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;

use freya_hooks::{use_applied_theme, IconTheme, IconThemeWith};

/// Properties for the [`CrossIcon`] component.
#[derive(Props, Clone, PartialEq)]
pub struct CrossIconProps {
    /// Theme override.
    pub theme: Option<IconThemeWith>,
    /// Color.
    #[props(into)]
    pub fill: String,
}

/// Icon component for a Cross.
#[allow(non_snake_case)]
pub fn CrossIcon(CrossIconProps { theme, fill }: CrossIconProps) -> Element {
    let IconTheme {
        height,
        width,
        margin,
    } = use_applied_theme!(&theme, icon);

    rsx!(svg {
        height: "{height}",
        width: "{width}",
        margin: "{margin}",
        svg_content: r#"
            <svg viewBox="0 0 292 292" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M0.0366211 268.737L268.737 0.0366211L291.365 22.664L22.664 291.365L0.0366211 268.737Z" fill="{fill}"/>
                <path d="M268.737 291.365L0.0366211 22.664L22.664 0.0366211L291.365 268.737L268.737 291.365Z" fill="{fill}"/>
            </svg>
        "#
    })
}
