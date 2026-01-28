#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    animation::*,
    prelude::*,
    router::prelude::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_title("Animated Router")))
}

fn app() -> impl IntoElement {
    router::<Route>(RouterConfig::default)
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppContainer)]
        #[route("/")]
        Home,
        #[route("/second")]
        SecondPage,
        #[route("/third")]
        ThirdPage,
}

#[derive(Clone, PartialEq)]
struct FromRouteToCurrent {
    from: Element,
    left_to_right: bool,
    area: State<Area>,
}

impl Component for FromRouteToCurrent {
    fn render(&self) -> impl IntoElement {
        let mut animated_router = use_animated_router::<Route>();
        let animations = use_animation_with_dependencies(
            &(self.left_to_right, self.from.clone()),
            move |conf, (left_to_right, _)| {
                conf.on_change(OnChange::Rerun);
                conf.on_creation(OnCreation::Run);

                let (start, end) = if *left_to_right { (1., 0.) } else { (0., 1.) };
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
            },
        );

        // Only render the destination route once the animation has finished
        use_side_effect(move || {
            if !*animations.is_running().read() && *animations.has_run_yet().read() {
                animated_router.write().settle();
            }
        });

        let (offset, scale_a, scale_b, corner_radius) = animations.get().value();
        let (scale_out, scale_in) = if self.left_to_right {
            (scale_a, scale_b)
        } else {
            (scale_b, scale_a)
        };

        let width = self.area.read().width();
        let offset = width - (offset * width);

        let to = outlet::<Route>().into_element();
        let (left, right) = if self.left_to_right {
            (self.from.clone(), to)
        } else {
            (to, self.from.clone())
        };

        rect()
            .expanded()
            .offset_x(-offset)
            .horizontal()
            .child(expanded(scale_out, corner_radius, left))
            .child(expanded(scale_in, corner_radius, right))
    }
}

fn expanded(scale: f32, corner_radius: f32, content: impl Into<Element>) -> Rect {
    rect()
        .width(Size::percent(100.))
        .height(Size::percent(100.))
        .center()
        .background((235, 235, 235))
        .scale(scale)
        .corner_radius(corner_radius)
        .child(content)
}

#[derive(Clone, PartialEq)]
struct AnimatedOutlet;

impl Component for AnimatedOutlet {
    fn render(&self) -> impl IntoElement {
        let mut area = use_state(Area::default);
        let animated_router = use_animated_router();

        let from_route = match &*animated_router.read() {
            AnimatedRouterContext::FromTo(Route::Home, Route::SecondPage) => {
                Some((Home.into_element(), true))
            }
            AnimatedRouterContext::FromTo(Route::Home, Route::ThirdPage) => {
                Some((Home.into_element(), true))
            }
            AnimatedRouterContext::FromTo(Route::SecondPage, Route::Home) => {
                Some((SecondPage.into_element(), false))
            }
            AnimatedRouterContext::FromTo(Route::SecondPage, Route::ThirdPage) => {
                Some((SecondPage.into_element(), true))
            }
            AnimatedRouterContext::FromTo(Route::ThirdPage, Route::Home) => {
                Some((ThirdPage.into_element(), false))
            }
            AnimatedRouterContext::FromTo(Route::ThirdPage, Route::SecondPage) => {
                Some((ThirdPage.into_element(), false))
            }
            _ => None,
        };

        rect()
            .on_sized(move |e: Event<SizedEventData>| area.set(e.area))
            .child(match from_route {
                Some((from, left_to_right)) => FromRouteToCurrent {
                    left_to_right,
                    from,
                    area,
                }
                .into_element(),
                None => expanded(1., 0., outlet::<Route>()).into_element(),
            })
    }
}

#[derive(Clone, PartialEq)]
struct AppContainer;

impl Component for AppContainer {
    fn render(&self) -> impl IntoElement {
        NativeRouter::new().child(AnimatedRouter::<Route>::new(
            rect()
                .content(Content::Flex)
                .child(
                    rect()
                        .theme_background()
                        .height(Size::flex(1.))
                        .child(AnimatedOutlet),
                )
                .child(
                    rect()
                        .horizontal()
                        .center()
                        .width(Size::fill())
                        .padding(8.)
                        .spacing(8.)
                        .child(bottom_tab(Route::Home, true, "Home Page 👋"))
                        .child(bottom_tab(Route::SecondPage, true, "Second Page 🌌"))
                        .child(bottom_tab(Route::ThirdPage, true, "Third Page 🦀")),
                ),
        ))
    }
}

fn bottom_tab<R: Routable + PartialEq>(route: R, exact: bool, label: &'static str) -> Link {
    Link::new(route.clone())
        .child(ActivableRoute::new(route.clone(), FloatingTab::new().child(label)).exact(exact))
}

#[derive(PartialEq)]
struct Home;

impl Component for Home {
    fn render(&self) -> impl IntoElement {
        label().text("👋 Hello, World!")
    }
}

#[derive(PartialEq)]
struct SecondPage;

impl Component for SecondPage {
    fn render(&self) -> impl IntoElement {
        label().text("🌌 Second Page")
    }
}

#[derive(PartialEq)]
struct ThirdPage;

impl Component for ThirdPage {
    fn render(&self) -> impl IntoElement {
        label().text("🦀 Third Page")
    }
}
