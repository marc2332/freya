#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_router::prelude::*;
use freya::prelude::*;

fn main() {
    launch_with_props(app, "Router Example", (550.0, 400.0));
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
                SidebarItem::<Route> {
                    to: Route::Home,
                    "Go to Hey ! 👋"
                },
                SidebarItem::<Route> {
                    to: Route::Wow,
                    "Go to Wow! 👈"
                },
                SidebarItem::<Route> {
                    onclick: |_| println!("Hello!"),
                    "Print Hello! 👀"
                }
            ),
            rect {
                main_align: "center",
                cross_align: "center",
                width: "100%",
                height: "100%",
                Outlet::<Route> {  }
            }
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
