use dioxus::prelude::*;
use freya_components::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::use_theme;

#[derive(Props)]
pub struct TabsBarProps<'a> {
    pub children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn TabsBar<'a>(cx: Scope<'a, TabsBarProps<'a>>) -> Element<'a> {
    let theme = use_theme(cx);
    let button_theme = &theme.read().button;
    render!(
        container {
            background: "{button_theme.background}",
            direction: "horizontal",
            height: "35",
            width: "100%",
            color: "{button_theme.font_theme.color}",
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
    let theme = use_theme(cx);
    let button_theme = &theme.read().button;

    let background = use_state(cx, || <&str>::clone(&button_theme.background));
    let set_background = background.setter();

    use_effect(cx, &button_theme.clone(), move |button_theme| async move {
        set_background(button_theme.background);
    });

    let content = cx.props.label;
    render!(
        container {
            background: "{background}",
            onmouseover: move |_| {
                    background.set(theme.read().button.hover_background);
            },
            onmouseleave: move |_| {
                background.set(theme.read().button.background);
            },
            width: "125",
            radius: "7",
            height: "100%",
            color: "{button_theme.font_theme.color}",
            RouterLink {
                to: cx.props.to,
                container {
                    width: "100%",
                    height: "100%",
                    padding: "7.5",
                    label {
                        align: "center",
                        height: "100%",
                        width: "100%",
                        content
                    }
                }
            }
        }
    )
}
