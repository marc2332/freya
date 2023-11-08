#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_router::prelude::*;
use freya::prelude::*;

fn main() {
    launch(app);
}

#[allow(non_snake_case)]
#[component]
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
            rect {
                overflow: "clip",
                width: "200",
                height: "100%",
                background: "rgb(20, 20, 20)",
                corner_radius: "0 7 0 7",
                padding: "20",
                color: "{color}",
                ScrollView {
                    padding: "10",
                    sidebar
                }
            }
            rect {
                overflow: "clip",
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
#[component]
fn SidebarItem<'a>(
    cx: Scope<'a>,
    children: Element<'a>,
    onclick: Option<EventHandler<'a, ()>>,
    to: Option<Route>,
) -> Element<'a> {
    let theme = use_get_theme(cx);
    let status = use_state(cx, ButtonStatus::default);
    let navigator = use_navigator(cx);

    let onclick = move |_| {
        if let Some(to) = to {
            navigator.replace(to.clone());
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
        rect {
            overflow: "clip",
            margin: "5 0",
            onclick: onclick,
            onmouseenter: onmouseenter,
            onmouseleave: onmouseleave,
            width: "100%",
            height: "auto",
            color: "{color}",
            shadow: "0 2 10 1 rgb(0, 0, 0, 45)",
            corner_radius: "10",
            padding: "12",
            background: "{background}",
            label {
                children
            }
        }
    )
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppSidebar)]
        #[route("/")]
        Home,

        #[route("/wow")]
        Wow,
    #[end_layout]
    #[route("/..route")]
    PageNotFound { },
}

#[allow(non_snake_case)]
fn AppSidebar(cx: Scope) -> Element {
    render!(
        Sidebar {
            sidebar: render!(
                SidebarItem {
                    to: Route::Home,
                    "Go to Hey ! 👋"
                },
                SidebarItem {
                    to: Route::Wow,
                    "Go to Wow! 👈"
                },
                SidebarItem {
                    onclick: |_| println!("Hello!"),
                    "Print Hello! 👀"
                }
            ),
            Outlet::<Route> {  }
        }
    )
}

#[allow(non_snake_case)]
fn Home(cx: Scope) -> Element {
    render!(
        label {
            "Just some text 😗 in /"
        }
    )
}

#[allow(non_snake_case)]
fn Wow(cx: Scope) -> Element {
    render!(
        label {
            "Just more text 👈!! in /wow"
        }
    )
}

#[allow(non_snake_case)]
fn PageNotFound(cx: Scope) -> Element {
    render!(
        label {
            "404!! 😵"
        }
    )
}

fn app(cx: Scope) -> Element {
    use_init_theme(cx, DARK_THEME);
    render!(Router::<Route> {})
}
