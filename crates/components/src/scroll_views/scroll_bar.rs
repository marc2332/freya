use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;

use freya_hooks::{use_applied_theme, ScrollBarTheme, ScrollBarThemeWith};

/// Properties for the [`ScrollBar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ScrollBarProps {
    /// Theme override.
    pub theme: Option<ScrollBarThemeWith>,
    pub children: Element,
    #[props(into)]
    pub width: String,
    #[props(into)]
    pub height: String,
    #[props(default = "0".to_string(), into)]
    pub offset_x: String,
    #[props(default = "0".to_string(), into)]
    pub offset_y: String,
    pub clicking_scrollbar: bool,
}

enum ScrollBarState {
    Idle,
    Hovering,
}

/// Scroll bar used for [`crate::ScrollView`] and [`crate::VirtualScrollView`].
#[allow(non_snake_case)]
pub fn ScrollBar(
    ScrollBarProps {
        width,
        height,
        clicking_scrollbar,
        offset_x,
        offset_y,
        theme,
        children,
    }: ScrollBarProps,
) -> Element {
    let mut status = use_signal(|| ScrollBarState::Idle);
    let ScrollBarTheme { background, .. } = use_applied_theme!(&theme, scroll_bar);

    let onmouseenter = move |_| status.set(ScrollBarState::Hovering);
    let onmouseleave = move |_| status.set(ScrollBarState::Idle);

    let background = match *status.read() {
        _ if clicking_scrollbar => background.as_ref(),
        ScrollBarState::Hovering => background.as_ref(),
        ScrollBarState::Idle => "transparent",
    };

    rsx!(
        rect {
            overflow: "clip",
            role: "scrollBar",
            width: "{width}",
            height: "{height}",
            offset_x: "{offset_x}",
            offset_y: "{offset_y}",
            background: "{background}",
            onmouseenter,
            onmouseleave,
            {children}
        }
    )
}
