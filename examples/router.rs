#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_router::prelude::{use_route, Outlet, Routable, Router};
use freya::prelude::*;

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
        Home,
        #[route("/wow")]
        Wow,
        #[route("/crab")]
        Crab,
    #[end_layout]
    #[route("/..route")]
    PageNotFound { },
}

#[derive(Clone)]
enum AnimatedRouterContext<R: Routable + PartialEq + Clone> {
    FromTo(R, R),
    In(R),
}

impl<R: Routable + PartialEq + Clone> AnimatedRouterContext<R> {
    pub fn target_route(&self) -> &R {
        match self {
            Self::FromTo(_, to) => to,
            Self::In(to) => to,
        }
    }

    pub fn set_target_route(&mut self, to: R) {
        match self {
            Self::FromTo(old_from, old_to) => {
                *old_from = old_to.clone(); 
                *old_to = to
            },
            Self::In(old_to) => *self = Self::FromTo(old_to.clone(), to),
        }
    }

    pub fn settle(&mut self) {
        match self {
            Self::FromTo(_, to) => *self = Self::In(to.clone()),
            _ => {}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct AnimatedRouterProps {
    children: Element,
}

fn AnimatedRouter<R: Routable + PartialEq + Clone>(
    AnimatedRouterProps { children }: AnimatedRouterProps,
) -> Element {
    let route = use_route::<R>();
    let mut prev_route = use_signal(|| AnimatedRouterContext::In(route.clone()));
    use_context_provider(move || prev_route);

    if prev_route.peek().target_route() != &route {
        prev_route.write().set_target_route(route);
    }

    rsx!({ children })
}

#[component]
fn FromRouteToCurrent(from: Element, upwards: bool) -> Element {
    let mut animated_router = use_context::<Signal<AnimatedRouterContext<Route>>>();
    let (attr, node_size) = use_node();
    let animations = use_animation_with_dependencies(&upwards, move |ctx, _| {
        let (start, end) = if upwards {
            (1., 0.)
        } else {
            (0., 1.)
        };
        ctx.with(
            AnimNum::new(start, end)
                .time(500)
                .ease(Ease::Out)
                .function(Function::Expo),
        )
    });

    use_memo(move || {
        if !animations.is_running() && animations.has_run_yet() {
            animated_router.write().settle();
        }
    });

    use_memo(use_reactive(&upwards, move |_| {
        animations.run(AnimDirection::Forward)
    }));

    let offset = animations.get().read().as_f32();

    let height = node_size.area.height();

    let offset = height - (offset * height);

    let (top, bottom) = if upwards {
        (from, rsx!(
            Outlet::<Route> { }
        )) 
    } else {
        (rsx!(
            Outlet::<Route> { }
        ), from)
    };

    rsx!(
        rect {
            reference: attr,
            height: "fill",
            width: "fill",
            offset_y: "-{offset}",
            rect {
                height: "100%",
                width: "100%",
                main_align: "center",
                cross_align: "center",
                {top}
            }
            rect {
                height: "100%",
                width: "100%",
                main_align: "center",
                cross_align: "center",
                {bottom}
            }
        }
    )
}

#[component]
fn AnimatedOutlet(children: Element) -> Element {
    let animated_router = use_context::<Signal<AnimatedRouterContext<Route>>>();

    match animated_router() {
        AnimatedRouterContext::FromTo(Route::Home, Route::Wow) => {
            rsx!(FromRouteToCurrent {
                upwards: true,
                from: rsx!(
                    Home {}
                ) 
            })
        }
        AnimatedRouterContext::FromTo(Route::Wow, Route::Home) => {
            rsx!(FromRouteToCurrent {
                upwards: false,
                from: rsx!(
                    Wow {}
                ) 
            })
        }
        AnimatedRouterContext::FromTo(Route::Wow, Route::Crab) => {
            rsx!(FromRouteToCurrent {
                upwards: true,
                from: rsx!(
                    Wow {}
                ) 
            })
        }
        AnimatedRouterContext::FromTo(Route::Home, Route::Crab) => {
            rsx!(FromRouteToCurrent {
                upwards: true,
                from: rsx!(
                    Home {}
                ) 
            })
        }
        AnimatedRouterContext::FromTo(Route::Crab, Route::Home) => {
            rsx!(FromRouteToCurrent {
                upwards: false,
                from: rsx!(
                    Crab {}
                ) 
            })
        }
        AnimatedRouterContext::FromTo(Route::Crab, Route::Wow) => {
            rsx!(FromRouteToCurrent {
                upwards: false,
                from: rsx!(
                    Crab {}
                ) 
            })
        }
        _ => {
            rsx!(
                rect {
                    main_align: "center",
                    cross_align: "center",
                    width: "fill",
                    height: "fill",
                    Outlet::<Route> {}
                }
            )
        },
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
                        SidebarItem {
                            onclick: |_| println!("Hello!"),
                            label {
                                "Print Hello! 👀"
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
