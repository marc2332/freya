use dioxus::prelude::*;
use dioxus_router::prelude::*;
use freya_components::{ButtonStatus, ScrollView};
use freya_elements::elements as dioxus_elements;
use freya_hooks::{theme_with, use_get_theme, ScrollViewThemeWith};

use crate::Route;

#[derive(Props, Clone, PartialEq)]
pub struct TabsBarProps {
    pub children: Element,
}

#[allow(non_snake_case)]
pub fn TabsBar(props: TabsBarProps) -> Element {
    rsx!(
        ScrollView {
            direction: "horizontal",
            theme: theme_with!(ScrollViewTheme {
                height : "35".into(),
            }),
            {props.children}
        }
    )
}

#[derive(Props, Clone, PartialEq)]
pub struct TabButtonProps {
    pub to: Route,
    pub label: String,
}

#[allow(non_snake_case)]
pub fn TabButton(props: TabButtonProps) -> Element {
    let router = use_navigator();
    let theme = use_get_theme();
    let mut status = use_signal(ButtonStatus::default);

    let onclick = move |_| {
        router.replace(props.to.clone());
    };

    let onmouseover = move |_| {
        if *status.read() != ButtonStatus::Hovering {
            status.set(ButtonStatus::Hovering);
        }
    };

    let onmouseleave = move |_| {
        status.set(ButtonStatus::default());
    };

    let background = match *status.read() {
        ButtonStatus::Hovering => theme.button.hover_background,
        ButtonStatus::Idle => theme.button.background,
    };
    let color = theme.button.font_theme.color;
    let border_fill = theme.button.border_fill;

    rsx!(
        rect {
            margin: "2",
            overflow: "clip",
            background: "{background}",
            onclick,
            onmouseover,
            onmouseleave,
            corner_radius: "7",
            height: "100%",
            color: "{color}",
            padding: "6 14",
            shadow: "0 4 5 0 rgb(0, 0, 0, 0.3)",
            border: "1 solid {border_fill}",
            main_align: "center",
            label {
                {props.label}
            }
        }
    )
}
