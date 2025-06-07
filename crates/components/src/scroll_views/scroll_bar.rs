use dioxus::prelude::*;
use freya_elements::{
    self as dioxus_elements,
};
use freya_hooks::{
    use_applied_theme,
    ScrollBarTheme,
    ScrollBarThemeWith,
};

/// Properties for the [`ScrollBar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ScrollBarProps {
    /// Theme override.
    pub theme: Option<ScrollBarThemeWith>,
    pub children: Element,
    #[props(into)]
    pub size: String,
    #[props(default = 0., into)]
    pub offset_x: f32,
    #[props(default = 0., into)]
    pub offset_y: f32,
    pub clicking_scrollbar: bool,
    #[props(default = false)]
    pub is_vertical: bool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ScrollBarState {
    Idle,
    Hovering,
}

/// Scroll bar used for [`crate::ScrollView`] and [`crate::VirtualScrollView`].
#[allow(non_snake_case)]
pub fn ScrollBar(
    ScrollBarProps {
        size,
        clicking_scrollbar,
        offset_x: inner_offset_x,
        offset_y: inner_offset_y,
        theme,
        children,
        is_vertical,
    }: ScrollBarProps,
) -> Element {
    let mut status = use_signal(|| ScrollBarState::Idle);
    let ScrollBarTheme { background, .. } = use_applied_theme!(&theme, scroll_bar);

    let onmouseenter = move |_| status.set(ScrollBarState::Hovering);
    let onmouseleave = move |_| status.set(ScrollBarState::Idle);

    let (inner_size, opacity) = match *status.read() {
        _ if clicking_scrollbar => (size.as_str(), 225.),
        ScrollBarState::Idle => ("5", 0.),
        ScrollBarState::Hovering => (size.as_str(), 225.),
    };

    let (offset_x, offset_y, width, height, inner_width, inner_height) = if is_vertical {
        (
            size.as_str(),
            "0",
            size.as_str(),
            "fill",
            inner_size,
            "auto",
        )
    } else {
        (
            "0",
            size.as_str(),
            "fill",
            size.as_str(),
            "auto",
            inner_size,
        )
    };

    rsx!(
        rect {
            width,
            height,
            layer: "-999",
            offset_x: "-{offset_x}",
            offset_y: "-{offset_y}",
            rect {
                onmouseenter,
                onmouseleave,
                a11y_role: "scroll-bar",
                width: "fill",
                height: "fill",
                background: "{background}",
                background_opacity: "{opacity}",
                direction: if is_vertical { "vertical" } else { "horizontal" },
                cross_align: "end",
                rect {
                    width: inner_width,
                    height: inner_height,
                    offset_x: "{inner_offset_x}",
                    offset_y: "{inner_offset_y}",
                    {children}
                }
            }
        }
    )
}
