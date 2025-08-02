#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_core::custom_attributes::NodeReferenceLayout;
use freya_router::prelude::*;

fn main() {
    launch_with_props(app, "Animated Slide Router", (650.0, 500.0));
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
        #[route("/settings")]
        Settings,
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
            let (start_offset, end_offset) = if left_to_right { (1., 0.) } else { (0., 1.) };
            let (start_background, end_background) = if left_to_right {
                ("transparent", "rgb(0, 0, 0, 0.2)")
            } else {
                ("rgb(0, 0, 0, 0.2)", "transparent")
            };
            (
                AnimNum::new(start_offset, end_offset)
                    .time(650)
                    .ease(Ease::Out)
                    .function(Function::Expo),
                AnimColor::new(start_background, end_background)
                    .time(650)
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

    let (offset, background) = animations.read().value();
    let width = node_size.read().area.width();

    let offset = offset * width;
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
            direction: "horizontal",
            View { {left} }
            rect {
                height: "100%",
                width: "100%",
                offset_x: "{offset}",
                position: "absolute",
                position_left: "0",
                position_top: "0",
                layer: "-999",
                background,
                View { {right} }
            }
        }
    )
}

#[component]
fn View(children: Element) -> Element {
    rsx!(
        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "rgb(240, 240, 240)",
            shadow: "0 2 4 0 rgb(0, 0, 0, 0.15)",
            {children}
        }
    )
}

#[component]
fn AnimatedOutlet(children: Element) -> Element {
    let (reference, node_size) = use_node_signal();
    let animated_router = use_context::<Signal<AnimatedRouterContext<Route>>>();

    let from_route = match animated_router() {
        AnimatedRouterContext::FromTo(Route::Home, Route::Settings) => Some((rsx!(Home {}), true)),
        AnimatedRouterContext::FromTo(Route::Settings, Route::Home) => {
            Some((rsx!(Settings {}), false))
        }
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
                View {
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
                AnimatedOutlet { }
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn Home() -> Element {
    rsx!(
        Link {
            to: Route::Settings,
            FilledButton {
                theme: theme_with!(ButtonTheme {
                    padding: "10 12".into(),
                    corner_radius: "100".into()
                }),
                label {
                    "Open Settings"
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn Settings() -> Element {
    rsx!(
        Link {
            to: Route::Home,
            FilledButton {
                theme: theme_with!(ButtonTheme {
                    padding: "10 12".into(),
                    corner_radius: "100".into()
                }),
                label {
                    "Close Settings"
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn Crab() -> Element {
    rsx!(
        label {
            "Other"
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn PageNotFound() -> Element {
    rsx!(
        label {
            "404."
        }
    )
}
