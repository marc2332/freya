mod routes;

use freya::{
    animation::*,
    icons::lucide,
    material_design::FloatingTabRippleExt,
    prelude::*,
    router::*,
};
use routes::*;

pub fn app() -> impl IntoElement {
    Router::<Route>::new(RouterConfig::default)
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppTopBar)]
        #[route("/")]
        ScrollViewDemo,
        #[route("/widgets")]
        WidgetsDemo,
        #[route("/portal")]
        PortalDemo,
        #[route("/editor")]
        EditorDemo,
        #[route("/markdown")]
        MarkdownDemo,
}

const ROUTES: [Route; 5] = [
    Route::ScrollViewDemo,
    Route::WidgetsDemo,
    Route::PortalDemo,
    Route::EditorDemo,
    Route::MarkdownDemo,
];

fn route_index(route: &Route) -> usize {
    ROUTES.iter().position(|r| r == route).unwrap_or(0)
}

fn route_element(route: &Route) -> Element {
    match route {
        Route::ScrollViewDemo => ScrollViewDemo.into_element(),
        Route::WidgetsDemo => WidgetsDemo.into_element(),
        Route::PortalDemo => PortalDemo.into_element(),
        Route::EditorDemo => EditorDemo.into_element(),
        Route::MarkdownDemo => MarkdownDemo.into_element(),
    }
}

#[derive(PartialEq)]
struct AppTopBar;

impl Component for AppTopBar {
    fn render(&self) -> impl IntoElement {
        use_init_theme(light_theme);

        NativeRouter::new().child(AnimatedRouter::<Route>::new(
            rect()
                .content(Content::flex())
                .theme_background()
                .theme_color()
                .child(
                    rect()
                        .width(Size::fill())
                        .height(Size::flex(1.))
                        .padding((40., 0., 8., 0.))
                        .child(AnimatedOutlet),
                )
                .child(
                    rect()
                        .horizontal()
                        .width(Size::fill())
                        .main_align(Alignment::center())
                        .padding((4., 4., 20., 4.))
                        .spacing(4.)
                        .child(tab(Route::ScrollViewDemo, "Scroll", lucide::scroll_text))
                        .child(tab(
                            Route::WidgetsDemo,
                            "Widgets",
                            lucide::sliders_horizontal,
                        ))
                        .child(tab(Route::PortalDemo, "Portal", lucide::layers))
                        .child(tab(Route::EditorDemo, "Editor", lucide::code))
                        .child(tab(Route::MarkdownDemo, "Markdown", lucide::notebook_text)),
                ),
        ))
    }
}

fn tab(route: Route, label: &'static str, icon: fn() -> Bytes) -> ActivableRoute<Route> {
    let theme = get_theme_or_default();
    ActivableRoute::new(
        route.clone(),
        Link::new(route).child(
            FloatingTab::new().ripple().child(
                rect()
                    .center()
                    .spacing(2.)
                    .child(
                        svg(icon())
                            .stroke(theme.read().colors.text_primary)
                            .width(Size::px(18.))
                            .height(Size::px(18.)),
                    )
                    .child(label),
            ),
        ),
    )
    .exact(true)
}

fn animated_page(scale: f32, corner_radius: f32, content: impl Into<Element>) -> Rect {
    rect()
        .width(Size::percent(100.))
        .height(Size::percent(100.))
        .center()
        .theme_background()
        .scale(scale)
        .corner_radius(corner_radius)
        .child(content)
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
                        .time(500)
                        .ease(Ease::Out)
                        .function(Function::Expo),
                    AnimNum::new(1., 0.4)
                        .time(500)
                        .ease(Ease::Out)
                        .function(Function::Expo),
                    AnimNum::new(0.4, 1.)
                        .time(500)
                        .ease(Ease::Out)
                        .function(Function::Expo),
                    AnimNum::new(50., 0.)
                        .time(500)
                        .ease(Ease::Out)
                        .function(Function::Expo),
                )
            },
        );

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

        let to = Outlet::<Route>::new().into_element();
        let (left, right) = if self.left_to_right {
            (self.from.clone(), to)
        } else {
            (to, self.from.clone())
        };

        rect()
            .expanded()
            .offset_x(-offset)
            .horizontal()
            .child(animated_page(scale_out, corner_radius, left))
            .child(animated_page(scale_in, corner_radius, right))
    }
}

#[derive(Clone, PartialEq)]
struct AnimatedOutlet;

impl Component for AnimatedOutlet {
    fn render(&self) -> impl IntoElement {
        let mut area = use_state(Area::default);
        let mut animated_router = use_animated_router();
        let involves_scroll = matches!(
            &*animated_router.read(),
            AnimatedRouterContext::FromTo(from, to)
                if *from == Route::ScrollViewDemo || *to == Route::ScrollViewDemo
        );

        let from_route = if involves_scroll {
            animated_router.write().settle();
            None
        } else {
            match &*animated_router.read() {
                AnimatedRouterContext::FromTo(from, to) => {
                    let left_to_right = route_index(to) > route_index(from);
                    Some((route_element(from), left_to_right))
                }
                _ => None,
            }
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
                None => animated_page(1., 0., Outlet::<Route>::new()).into_element(),
            })
    }
}
