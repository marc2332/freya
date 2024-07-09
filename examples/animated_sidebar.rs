#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_router::prelude::{Outlet, Routable, Router};
use freya::prelude::*;

fn main() {
    launch_with_props(app, "Animated Sidebar", (650.0, 500.0));
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
        #[route("/crab")]
        Crab,
    #[end_layout]
    #[route("/..route")]
    PageNotFound { },
}

#[component]
fn FromRouteToCurrent(from: Element, upwards: bool) -> Element {
    let mut animated_router = use_animated_router::<Route>();
    let (reference, node_size) = use_node();
    let animations = use_animation_with_dependencies(&upwards, move |ctx, upwards| {
        let (start, end) = if upwards { (1., 0.) } else { (0., 1.) };
        ctx.with(
            AnimNum::new(start, end)
                .time(500)
                .ease(Ease::Out)
                .function(Function::Expo),
        )
    });

    // Only render the destination route once the animation has finished
    use_memo(move || {
        if !animations.is_running() && animations.has_run_yet() {
            animated_router.write().settle();
        }
    });

    // Run the animation when any prop changes
    use_memo(use_reactive((&upwards, &from), move |_| {
        animations.run(AnimDirection::Forward)
    }));

    let offset = animations.get().read().as_f32();
    let height = node_size.area.height();

    let offset = height - (offset * height);
    let to = rsx!(Outlet::<Route> {});
    let (top, bottom) = if upwards { (from, to) } else { (to, from) };

    rsx!(
        rect {
            reference,
            height: "fill",
            width: "fill",
            offset_y: "-{offset}",
            Expand { {top} }
            Expand { {bottom} }
        }
    )
}

#[component]
fn Expand(children: Element) -> Element {
    rsx!(
        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            {children}
        }
    )
}

#[component]
fn AnimatedOutlet(children: Element) -> Element {
    let animated_router = use_context::<Signal<AnimatedRouterContext<Route>>>();

    let from_route = match animated_router() {
        AnimatedRouterContext::FromTo(Route::Home, Route::Wow) => Some((rsx!(Home {}), true)),
        AnimatedRouterContext::FromTo(Route::Home, Route::Crab) => Some((rsx!(Home {}), true)),
        AnimatedRouterContext::FromTo(Route::Wow, Route::Home) => Some((rsx!(Wow {}), false)),
        AnimatedRouterContext::FromTo(Route::Wow, Route::Crab) => Some((rsx!(Wow {}), true)),
        AnimatedRouterContext::FromTo(Route::Crab, Route::Home) => Some((rsx!(Crab {}), false)),
        AnimatedRouterContext::FromTo(Route::Crab, Route::Wow) => Some((rsx!(Crab {}), false)),
        _ => None,
    };

    if let Some((from, upwards)) = from_route {
        rsx!(FromRouteToCurrent { upwards, from })
    } else {
        rsx!(
            Expand {
                Outlet::<Route> {}
            }
        )
    }
}

#[allow(non_snake_case)]
fn AppSidebar() -> Element {
    rsx!(
        NativeRouter {
            AnimatedRouter::<Route> {
                Sidebar {
                    sidebar: rsx!(
                        Link {
                            to: Route::Home,
                            ActivableRoute {
                                route: Route::Home,
                                exact: true,
                                SidebarItem {
                                    label {
                                        "Go to Hey ! 👋"
                                    }
                                },
                            }
                        },
                        Link {
                            to: Route::Wow,
                            ActivableRoute {
                                route: Route::Wow,
                                SidebarItem {
                                    label {
                                        "Go to Wow! 👈"
                                    }
                                },
                            }
                        },
                        Link {
                            to: Route::Crab,
                            ActivableRoute {
                                route: Route::Crab,
                                SidebarItem {
                                    label {
                                        "Go to Crab! 🦀"
                                    }
                                },
                            }
                        },
                    ),
                    Body {
                        AnimatedOutlet { }
                    }
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

#[allow(non_snake_case)]
#[component]
fn PageNotFound() -> Element {
    rsx!(
        label {
            "404!! 😵"
        }
    )
}
