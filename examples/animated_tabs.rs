#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_router::prelude::{
    Outlet,
    Routable,
    Router,
};
use freya::{
    common::NodeReferenceLayout,
    prelude::*,
};

fn main() {
    launch_with_props(app, "Animated Tabs Router", (650.0, 500.0));
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
fn FromRouteToCurrent(
    from: Element,
    left_to_right: bool,
    node_size: ReadOnlySignal<NodeReferenceLayout>,
) -> Element {
    let mut animated_router = use_animated_router::<Route>();
    let animations =
        use_animation_with_dependencies(&left_to_right, move |_conf, left_to_right| {
            let (start, end) = if left_to_right { (1., 0.) } else { (0., 1.) };
            AnimNum::new(start, end)
                .time(400)
                .ease(Ease::Out)
                .function(Function::Expo)
        });

    // Only render the destination route once the animation has finished
    use_memo(move || {
        if !animations.is_running() && animations.has_run_yet() {
            animated_router.write().settle();
        }
    });

    // Run the animation when any prop changes
    use_memo(use_reactive((&left_to_right, &from), move |_| {
        animations.run(AnimDirection::Forward)
    }));

    let offset = animations.get().read().read();
    let width = node_size.read().area.width();

    let offset = width - (offset * width);
    let to = rsx!(Outlet::<Route> {});
    let (left, right) = if left_to_right {
        (from, to)
    } else {
        (to, from)
    };

    rsx!(
        rect {
            height: "fill",
            width: "fill",
            offset_x: "-{offset}",
            direction: "horizontal",
            Expand { {left} }
            Expand { {right} }
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
    let (reference, node_size) = use_node_signal();
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

    rsx!(
        rect {
            reference,
            if let Some((from, left_to_right)) = from_route {
                FromRouteToCurrent {
                    left_to_right,
                    from,
                    node_size
                }
            } else {
                Expand {
                    Outlet::<Route> {}
                }
            }
        }
    )
}

#[allow(non_snake_case)]
fn AppSidebar() -> Element {
    rsx!(
        NativeRouter {
            AnimatedRouter::<Route> {
                rect {
                    height: "calc(100% - 50)",
                    width: "fill",
                    Body {
                        AnimatedOutlet { }
                    }
                }
                rect {
                    direction: "horizontal",
                    main_align: "center",
                    cross_align: "center",
                    width: "fill",
                    BottomTab {
                        route: Route::Home,
                        exact: true,
                        "Go to Hey ! 👋"
                    }
                    BottomTab {
                        route: Route::Wow,
                        "Go to Wow! 👈"
                    }
                    BottomTab {
                        route: Route::Crab,
                        "Go to Crab! 🦀"
                    }
                }

            }
        }
    )
}

#[derive(Props, PartialEq, Clone)]
struct BottomTabProps<R: Routable + PartialEq> {
    children: Element,
    route: R,
    #[props(default = false)]
    exact: bool,
}

#[allow(non_snake_case)]
fn BottomTab<R: Routable + PartialEq>(
    BottomTabProps {
        exact,
        children,
        route,
    }: BottomTabProps<R>,
) -> Element {
    rsx!(
        Link {
            to: route.clone(),
            ActivableRoute {
                route,
                exact,
                freya::components::BottomTab {
                    label {
                        main_align: "center",
                        {children}
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
