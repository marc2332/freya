use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;

use freya_hooks::{use_applied_theme, IconTheme, IconThemeWith};

/// Properties for the [`TickIcon`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TickIconProps {
    /// Theme override.
    pub theme: Option<IconThemeWith>,
    /// Color.
    #[props(into)]
    pub fill: String,
}

/// Icon component for a Tick.
#[allow(non_snake_case)]
pub fn TickIcon(TickIconProps { theme, fill }: TickIconProps) -> Element {
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
            <svg viewBox="0 0 333 263" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path d="M304.109 0L333 28.8909L99.1812 262.71L70.2903 233.819L304.109 0Z" fill="{fill}"/>
                <path d="M0 163.53L27.1003 136.429L126.003 235.332L98.9029 262.433L0 163.53Z" fill="{fill}"/>
            </svg>        
        "#
    })
}
