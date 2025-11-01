#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_router::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    router::<Route>(RouterConfig::default)
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppBotomBar)]
        #[route("/")]
        Home,
        #[route("/settings")]
        Settings,
}

#[derive(PartialEq)]
struct AppBotomBar;
impl Render for AppBotomBar {
    fn render(&self) -> Element {
        NativeRouter::new()
            .child(
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
            .into()
    }
}

#[derive(PartialEq)]
struct Home;
impl Render for Home {
    fn render(&self) -> Element {
        "Home Page!".into()
    }
}

#[derive(PartialEq)]
struct Settings;
impl Render for Settings {
    fn render(&self) -> Element {
        "Settings Page!".into()
    }
}
