#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_core::custom_attributes::NodeReferenceLayout;
use freya_router::prelude::*;

fn main() {
    launch_with_params(app, "Animated Tabs Router", (650.0, 500.0));
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
            (
                AnimNum::new(start, end)
                    .time(1000)
                    .ease(Ease::Out)
                    .function(Function::Expo),
                AnimNum::new(1., 0.2)
                    .time(1000)
                    .ease(Ease::Out)
                    .function(Function::Expo),
                AnimNum::new(0.2, 1.)
                    .time(1000)
                    .ease(Ease::Out)
                    .function(Function::Expo),
                AnimNum::new(100., 0.)
                    .time(1000)
                    .ease(Ease::Out)
                    .function(Function::Expo),
            )
        });

    // Run the animation when any prop changes
    use_memo(use_reactive((&left_to_right, &from), move |_| {
        animations.run(AnimDirection::Forward)
    }));

    // Only render the destination route once the animation has finished
    use_effect(move || {
        if !animations.is_running() && animations.has_run_yet() {
            animated_router.write().settle();
        }
    });

    let animations = animations.get()();
    let offset = animations.0.read();
    let (scale_out, scale_in) = if left_to_right {
        (animations.1.read(), animations.2.read())
    } else {
        (animations.2.read(), animations.1.read())
    };
    let corner_radius = animations.3.read();
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
            Expand { scale: scale_out, corner_radius, {left} }
            Expand { scale: scale_in, corner_radius, {right} }
        }
    )
}

#[component]
fn Expand(children: Element, scale: f32, corner_radius: f32) -> Element {
    rsx!(
        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "rgb(225, 225, 225)",
            corner_radius: "{corner_radius}",
            scale: "{scale}",
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
                    scale: 1.0,
                    corner_radius: 0.,
                    Outlet::<Route> {},
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
                    content: "flex",
                    Body {
                        height: "flex(1)",
                        AnimatedOutlet { }
                    }
                    rect {
                        direction: "horizontal",
                        main_align: "center",
                        cross_align: "center",
                        width: "fill",
                        padding: "8",
                        spacing: "8",
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
