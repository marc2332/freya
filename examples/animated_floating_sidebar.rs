#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::prelude::*;
use freya_router::prelude::*;

fn main() {
    launch_with_props(app, "Animated Sidebar Indicator", (650.0, 500.0));
}

fn app() -> Element {
    rsx!(
        GlobalAnimatedPositionProvider::<()> {
            Router::<Route> {}
        }
    )
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppSidebar)]
        #[route("/")]
        Home,
        #[route("/wow")]
        Wow,
        #[route("/crab")]
        Crab,
}

#[component]
fn BubbleIndicator() -> Element {
    let is_active = use_activable_route();
    if !is_active {
        return rsx!();
    }
    rsx!(
        rect {
            position: "absolute",
            offset_x: "-10",
            offset_y: "-8",
            GlobalAnimatedPosition::<()> {
                width: "calc(180 - 16)",
                height: "35",
                function: Function::Expo,
                duration: Duration::from_millis(600),
                id: (),
                rect {
                    width: "calc(180 - 16)",
                    height: "35",
                    corner_radius: "32",
                    background: "rgb(225, 225, 225)",
                    layer: "5",
                }
            }
        }
    )
}

#[allow(non_snake_case)]
fn SidebarItem(props: SidebarItemProps) -> Element {
    freya::prelude::SidebarItem(SidebarItemProps {
        theme: theme_with!(SidebarItemTheme {
            hover_background: "transparent".into()
        })
        .into(),
        overflow: "none".to_string(),
        ..props
    })
}

#[allow(non_snake_case)]
fn AppSidebar() -> Element {
    rsx!(
        Sidebar {
            sidebar: rsx!(
                Link {
                    to: Route::Home,
                    ActivableRoute {
                        route: Route::Home,
                        exact: true,
                        SidebarItem {
                            BubbleIndicator { }
                            label {
                                "Go to Hey ! 👋"
                            }
                        }
                    }
                }
                Link {
                    to: Route::Wow,
                    ActivableRoute {
                        route: Route::Wow,
                        SidebarItem {
                            BubbleIndicator { }
                            label {
                                "Go to Wow! 👈"
                            }
                        }
                    }
                }
                Link {
                    to: Route::Crab,
                    ActivableRoute {
                        route: Route::Crab,
                        SidebarItem {
                            BubbleIndicator { }
                            label {
                                "Go to Crab! 🦀"
                            }
                        }
                    }
                }
            ),
            Body {
                rect {
                    height: "100%",
                    width: "100%",
                    main_align: "center",
                    cross_align: "center",
                    Outlet::<Route> {}
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
fn Crab() -> Element {
    rsx!(
        label {
            "🦀🦀🦀🦀🦀 /crab"
        }
    )
}
