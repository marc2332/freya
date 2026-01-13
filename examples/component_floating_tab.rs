#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_router::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    router::<Route>(RouterConfig::default)
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppBottomBar)]
        #[route("/")]
        Home,
        #[route("/settings")]
        Settings,
}

#[derive(PartialEq)]
struct AppBottomBar;
impl Component for AppBottomBar {
    fn render(&self) -> impl IntoElement {
        NativeRouter::new().child(
            rect()
                .content(Content::flex())
                .child(
                    rect()
                        .width(Size::fill())
                        .height(Size::flex(1.))
                        .center()
                        .child(outlet::<Route>()),
                )
                .child(
                    rect()
                        .horizontal()
                        .width(Size::fill())
                        .main_align(Alignment::center())
                        .padding(8.)
                        .child(
                            ActivableRoute::new(
                                Route::Home,
                                Link::new(Route::Home).child(FloatingTab::new().child("Home")),
                            )
                            .exact(true),
                        )
                        .child(
                            ActivableRoute::new(
                                Route::Settings,
                                Link::new(Route::Settings)
                                    .child(FloatingTab::new().child("Settings")),
                            )
                            .exact(true),
                        ),
                ),
        )
    }
}

#[derive(PartialEq)]
struct Home;
impl Component for Home {
    fn render(&self) -> impl IntoElement {
        "Home Page!"
    }
}

#[derive(PartialEq)]
struct Settings;
impl Component for Settings {
    fn render(&self) -> impl IntoElement {
        "Settings Page!"
    }
}
