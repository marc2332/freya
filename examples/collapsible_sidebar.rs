#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    animation::*,
    icons::lucide,
    prelude::*,
    router::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    Router::<Route>::new(RouterConfig::default)
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppShell)]
        #[route("/")]
        Home,
        #[route("/settings")]
        Settings,
        #[route("/about")]
        About,
}

#[derive(PartialEq)]
struct AppShell;

impl Component for AppShell {
    fn render(&self) -> impl IntoElement {
        let theme = use_init_theme(light_theme);
        let surface = theme.read().colors.surface_tertiary;
        let icon_color = theme.read().colors.text_primary;
        let mut window_width = use_state(|| f32::MAX);
        let mut sidebar_open = use_state(|| false);
        let mut slide = use_animation(|_| {
            AnimNum::new(-210., 0.)
                .time(250)
                .ease(Ease::Out)
                .function(Function::Expo)
        });

        let compact = *window_width.read() < 800.;
        let sliding_sidebar = compact && (*sidebar_open.read() || *slide.is_running().read());

        let toggle_sidebar = move |_| {
            if *sidebar_open.peek() {
                slide.reverse();
                sidebar_open.set(false);
            } else {
                slide.start();
                sidebar_open.set(true);
            }
        };

        rect()
            .native_router()
            .expanded()
            .center()
            .theme_color()
            .theme_background()
            .on_sized(move |event: Event<SizedEventData>| {
                window_width.set_if_modified(event.area.width());
                if event.area.width() >= 800. && *sidebar_open.peek() {
                    sidebar_open.set(false);
                    slide.reset();
                }
            })
            .child(
                rect()
                    .width(Size::percent(85.))
                    .height(Size::percent(85.))
                    .horizontal()
                    .background(surface)
                    .corner_radius(20.)
                    .shadow((0., 10., 30., 0., (0, 0, 0, 40)))
                    .overflow(Overflow::Clip)
                    .maybe_child((!compact).then(|| sidebar().key("sidebar")))
                    .child(
                        rect()
                            .key("content")
                            .expanded()
                            .height(Size::fill())
                            .padding(if compact {
                                (24., 24., 24., 68.)
                            } else {
                                (24., 24., 24., 24.)
                            })
                            .child(Outlet::<Route>::new()),
                    )
                    .maybe_child(sliding_sidebar.then(|| {
                        sidebar()
                            .key("floating-sidebar")
                            .position(Position::new_absolute().left(slide.read().value()).top(0.))
                            .layer(Layer::Relative(100))
                            .padding((60., 12., 12., 12.))
                            .shadow((10., 0., 30., 0., (0, 0, 0, 40)))
                    }))
                    .maybe_child(compact.then(|| {
                        rect()
                            .key("burger")
                            .position(Position::new_absolute().left(12.).top(12.))
                            .layer(Layer::Relative(200))
                            .child(
                                Button::new().on_press(toggle_sidebar).child(
                                    SvgViewer::new(if *sidebar_open.read() {
                                        lucide::x()
                                    } else {
                                        lucide::menu()
                                    })
                                    .stroke(icon_color)
                                    .width(Size::px(20.))
                                    .height(Size::px(20.)),
                                ),
                            )
                    })),
            )
    }
}

fn sidebar() -> Rect {
    rect()
        .width(Size::px(210.))
        .height(Size::fill())
        .theme_background()
        .padding(12.)
        .spacing(4.)
        .child(
            rect()
                .padding((8., 8., 16., 8.))
                .font_size(20.)
                .font_weight(FontWeight::BOLD)
                .child("Collapsible sidebar"),
        )
        .child(item(Route::Home, "Home", true))
        .child(item(Route::Settings, "Settings", false))
        .child(item(Route::About, "About", false))
}

/// Sidebar entry that highlights itself while its route is active.
fn item(route: Route, title: &'static str, exact: bool) -> ActivableRoute<Route> {
    ActivableRoute::new(
        route.clone(),
        Link::new(route).child(SideBarItem::new().child(title)),
    )
    .exact(exact)
}

#[derive(PartialEq)]
struct Home;
impl Component for Home {
    fn render(&self) -> impl IntoElement {
        "Home Page! Resize the window below 800px to collapse the sidebar."
    }
}

#[derive(PartialEq)]
struct Settings;
impl Component for Settings {
    fn render(&self) -> impl IntoElement {
        "Settings Page!"
    }
}

#[derive(PartialEq)]
struct About;
impl Component for About {
    fn render(&self) -> impl IntoElement {
        "About Page!"
    }
}
