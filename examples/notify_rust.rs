#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_router::prelude::*;

fn main() {
    launch_with_props(app, "Router Example", (550.0, 400.0));
}

fn app() -> Element {
    rsx!(Router::<Route> {})
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppSidebar)]
        #[route("/")]
        Simple,
        #[route("/wow")]
    #[end_layout]
    #[route("/..route")]
    PageNotFound { },
}

#[allow(non_snake_case)]
fn AppSidebar() -> Element {
    rsx!(
        NativeRouter {
            Sidebar {
                sidebar: rsx!(
                    Link {
                        to: Route::Simple,
                        ActivableRoute {
                            route: Route::Simple,
                            exact: true,
                            SidebarItem {
                                label {
                                    "Go to Hey ! 👋"
                                }
                            }
                        }
                    }
                ),
                Body {
                    main_align: "center",
                    cross_align: "center",
                    width: "100%",
                    height: "100%",
                    Outlet::<Route> {  }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn Simple() -> Element {
    rsx!(
        label {
            "Simple -> /"
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn PageNotFound() -> Element {
    rsx!(
        label {
            "404!! 😵"
        }
    )
}
