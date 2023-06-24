use dioxus::prelude::*;
use dioxus_router::use_router;
use freya_components::{ButtonStatus, ScrollView};
use freya_elements::elements as dioxus_elements;
use freya_hooks::use_get_theme;

#[derive(Props)]
pub struct TabsBarProps<'a> {
    pub children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn TabsBar<'a>(cx: Scope<'a, TabsBarProps<'a>>) -> Element<'a> {
    render!(
        ScrollView {
            direction: "horizontal",
            height: "35",
            width: "100%",
            &cx.props.children
        }
    )
}

#[derive(Props)]
pub struct TabButtonProps<'a> {
    pub to: &'a str,
    pub label: &'a str,
}

#[allow(non_snake_case)]
pub fn TabButton<'a>(cx: Scope<'a, TabButtonProps<'a>>) -> Element<'a> {
    let router = use_router(cx);
    let theme = use_get_theme(cx);
    let status = use_state(cx, ButtonStatus::default);

    let onclick = move |_| {
        router.replace_route(cx.props.to, None, None);
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
    let content = cx.props.label;

    render!(
        rect {
            overflow: "clip",
            background: "{background}",
            onclick: onclick,
            onmouseover: onmouseover,
            onmouseleave: onmouseleave,
            width: "125",
            radius: "7",
            height: "100%",
            color: "{color}",
            padding: "7.5",
            label {
                align: "center",
                height: "100%",
                width: "100%",
                content
            }
        }
    )
}
