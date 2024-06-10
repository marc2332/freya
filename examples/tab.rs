#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_router::prelude::{
    Outlet,
    Routable,
    Router,
};
use freya::prelude::*;

fn main() {
    launch_with_props(app, "Tabs", (500.0, 350.0));
}

fn app() -> Element {
    rsx!(Router::<Route> {})
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppSidebar)]
        #[route("/")]
        Home,
        #[route("/wow")]
        Wow,
        #[route("/settings")]
        Settings,
    #[end_layout]
    #[route("/..route")]
    PageNotFound { },
}

#[allow(non_snake_case)]
fn AppSidebar() -> Element {
    rsx!(
        NativeRouter {
            TabsBar {
                Link {
                    to: Route::Home,
                    ActivableRoute {
                        route: Route::Home,
                        exact: true,
                        Tab {
                            label {
                                "Go to Hey ! 👋"
                            }
                        }
                    }
                },
                Link {
                    to: Route::Wow,
                    ActivableRoute {
                        route: Route::Wow,
                        Tab {
                            label {
                                "Go to Wow! 👈"
                            }
                        }
                    }
                },
                Link {
                    to: Route::Settings,
                    ActivableRoute {
                        route: Route::Settings,
                        Tab {
                            label {
                                "Go to Settings! 🧰"
                            }
                        },
                    }
                },
            }
            Body {
                rect {
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
fn Home() -> Element {
    rsx!(
        label {
            "Just some text 😗 in /"
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn Wow() -> Element {
    rsx!(
        label {
            "Just more text 👈!! in /wow"
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn Settings() -> Element {
    rsx!(
        label {
            "Maybe some settings🧰!! in /settings"
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
