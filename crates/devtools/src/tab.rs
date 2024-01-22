use dioxus::prelude::*;
use dioxus_router::prelude::*;
use freya_components::{ButtonStatus, ScrollView};
use freya_elements::elements as dioxus_elements;
use freya_hooks::{theme_with, use_get_theme, ScrollViewThemeWith};

use crate::Route;

#[derive(Props)]
pub struct TabsBarProps<'a> {
    pub children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn TabsBar<'a>(cx: Scope<'a, TabsBarProps<'a>>) -> Element<'a> {
    rsx!(
        ScrollView {
            direction: "horizontal",
            theme: theme_with!(ScrollViewTheme {
                height : "35".into(),
            }),
            &cx.props.children
        }
    )
}

#[derive(Props)]
pub struct TabButtonProps<'a> {
    pub to: Route,
    pub label: &'a str,
}

#[allow(non_snake_case)]
pub fn TabButton<'a>(cx: Scope<'a, TabButtonProps<'a>>) -> Element<'a> {
    let router = use_navigator(cx);
    let theme = use_get_theme(cx);
    let status = use_state(cx, ButtonStatus::default);

    let onclick = |_| {
        router.replace(cx.props.to.clone());
    };

    let onmouseover = move |_| {
        if *status.get() != ButtonStatus::Hovering {
            status.set(ButtonStatus::Hovering);
        }
    };

    let onmouseleave = move |_| {
        status.set(ButtonStatus::default());
    };

    let background = match *status.get() {
        ButtonStatus::Hovering => theme.button.hover_background,
        ButtonStatus::Idle => theme.button.background,
    };
    let color = theme.button.font_theme.color;
    let border_fill = theme.button.border_fill;
    let content = cx.props.label;

    rsx!(
        rect {
            margin: "2",
            overflow: "clip",
            background: "{background}",
            onclick: onclick,
            onmouseover: onmouseover,
            onmouseleave: onmouseleave,
            corner_radius: "7",
            height: "100%",
            color: "{color}",
            padding: "6 14",
            shadow: "0 4 5 0 rgb(0, 0, 0, 0.3)",
            border: "1 solid {border_fill}",
            main_align: "center",
            label {
                content
            }
        }
    )
}
