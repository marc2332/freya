#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_router::prelude::*;
use freya::prelude::*;

fn main() {
    launch_with_props(app, "Router Example", (550.0, 400.0));
}

fn app() -> Element {
    rsx!(ThemeProvider { theme: DARK_THEME, Router::<Route> {} })
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
fn AppSidebar() -> Element {
    rsx!(
        Sidebar {
            sidebar: rsx!(
                SidebarItem {
                    //Link {
                        //to: Route::Home,
                        "Go to Hey ! 👋"
                    //}
                },
                SidebarItem {
                    //Link {
                        //to: Route::Wow,
                        "Go to Wow! 👈"
                    //}
                },
                SidebarItem {
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

#[component]
fn Home() -> Element {
    rsx!(
        label {
            "Just some text 😗 in /"
        }
    )
}

#[component]
fn Wow() -> Element {
    rsx!(
        label {
            "Just more text 👈!! in /wow"
        }
    )
}

#[component]
fn PageNotFound() -> Element {
    rsx!(
        label {
            "404!! 😵"
        }
    )
}
