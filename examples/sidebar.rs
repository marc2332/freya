#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use dioxus_router::*;

fn main() {
    launch(app);
}

#[allow(non_snake_case)]
#[inline_props]
fn Sidebar<'a>(cx: Scope<'a>, children: Element<'a>, sidebar: Element<'a>) -> Element<'a> {
    let theme = use_theme(cx);
    let background = theme.read().body.background;
    let color = theme.read().body.color;

    render!(
        rect {
            width: "100%",
            height: "100%",
            direction: "horizontal",
            background: "{background}",
            container {
                width: "200",
                height: "100%",
                background: "rgb(50, 50, 50)",
                radius: "0 7 0 7",
                padding: "30",
                color: "{color}",
                ScrollView {
                    sidebar
                }
            }
            container {
                width: "calc(100% - 200)",
                height: "100%",
                padding: "30",
                color: "{color}",
                children,
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn SidebarItem<'a>(cx: Scope<'a>, children: Element<'a>, onclick: Option<EventHandler<'a, ()>>, to: Option<&'a str>) -> Element<'a> {
    let theme = use_get_theme(cx);
    let status = use_state(cx, ButtonStatus::default);
    let router = use_router(cx);

    let onclick = move |_| {
        if let Some(to) = to {
            router.replace_route(to, None, None)
        }
        if let Some(onclick) = onclick {
            onclick.call(());
        }
    };

    let onmouseenter = move |_| {
        status.set(ButtonStatus::Hovering);
    };

    let onmouseleave = move |_| {
        status.set(ButtonStatus::default());
    };

    let background = match *status.get() {
        ButtonStatus::Hovering => theme.button.hover_background,
        ButtonStatus::Idle => theme.button.background,
    };
    let color = theme.button.font_theme.color;

    render!(
        container {
            margin: "5 0",
            onclick: onclick,
            onmouseenter: onmouseenter,
            onmouseleave: onmouseleave,
            width: "100%",
            height: "auto",
            direction: "both",
            color: "{color}",
            shadow: "0 5 15 10 black",
            radius: "10",
            padding: "12",
            background: "{background}",
            label {
                children
            }
        }
    )
}

fn app(cx: Scope) -> Element {
    use_init_default_theme(cx);
    render!(
        Router {
            Sidebar {
                sidebar: render!(
                    SidebarItem {
                        to: "/",
                        "Go to Hey ! 👋"
                    }
                    SidebarItem {
                        to: "/wow",
                        "Go to Wow! 👈"
                    }
                    SidebarItem {
                        onclick: |_| println!("whatever"),
                        "Idk ! 🦀"
                    }
                )
                Route {
                    to: "/",
                    label {
                        "Just some text 😗 in /"
                    }
                }
                Route {
                    to: "/wow",
                    label {
                        "Just more text 👈!! in /wow"
                    }
                }
            }
        }
    )
}
