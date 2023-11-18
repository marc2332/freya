#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_router::prelude::*;
use freya::prelude::*;

fn main() {
    launch_with_props(app, "Freya Gallery", (700.0, 500.0));
}

fn app(cx: Scope) -> Element {
    render!(Router::<Route> {})
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppSidebar)]
        #[route("/")]
        Home,
        #[route("/button")]
        ButtonDemo,
        #[route("/scrollview")]
        ScrollViewDemo,
    #[end_layout]
    #[route("/..route")]
    PageNotFound { },
}

#[allow(non_snake_case)]
fn AppSidebar(cx: Scope) -> Element {
    render!(
        Sidebar {
            sidebar: render!(
                SidebarItem::<Route> {
                    to: Route::Home,
                    "Introduction"
                },
                SmallText {
                    "Components"
                }
                SidebarItem::<Route> {
                    to: Route::ButtonDemo,
                    "Button"
                },
                SidebarItem::<Route> {
                    to: Route::ScrollViewDemo,
                    "ScrollView"
                }
            ),
            rect {
                width: "100%",
                height: "100%",
                padding: "20",
                Outlet::<Route> {  }
            }
        }
    )
}

#[allow(non_snake_case)]
fn Home(cx: Scope) -> Element {
    render!(
        label {
            font_size: "40",
            "Freya"
        }
        label {
            "Freya is a native GUI library for Rust. Powered by Dioxus and Skia."
        }
    )
}

#[allow(non_snake_case)]
fn ButtonDemo(cx: Scope) -> Element {
    let mut count = use_state(cx, || 4);

    render!(
        label {
            font_size: "40",
            "Button"
        }
        label {
            "Example:"
        }
        DemoBlock {
            Button {
                onclick: move |_| count += 1,
                label {
                    "{count}"
                }
            }
        }
    )
}

#[allow(non_snake_case)]
fn ScrollViewDemo(cx: Scope) -> Element {
    render!(
        label {
            font_size: "40",
            "ScrollView"
        }
        label {
            "Example:"
        }

    )
}

#[allow(non_snake_case)]
#[inline_props]
fn SmallText<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    render!(label {
        font_size: "14",
        margin: "4 0 2 0",
        children
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn DemoBlock<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    render!(rect {
        main_align: "center",
        cross_align: "center",
        width: "100%",
        padding: "30",
        background: "rgb(240, 240, 240)",
        corner_radius: "16",
        margin: "2 0",
        children
    })
}

#[allow(non_snake_case)]
fn PageNotFound(cx: Scope) -> Element {
    render!(
        label {
            "404!! 😵"
        }
    )
}
