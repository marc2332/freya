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
    #[layout(AppSideBar)]
        #[route("/")]
        Home,
        #[route("/settings")]
        Settings,
}

#[derive(PartialEq)]
struct AppSideBar;
impl Render for AppSideBar {
    fn render(&self) -> Element {
        NativeRouter::new()
            .child(
                SideBar::new()
                    .bar(
                        rect()
                            .child(
                                ActivableRoute::new(
                                    Route::Home,
                                    Link::new(Route::Home).child(SideBarItem::new().child("Home")),
                                )
                                .exact(true),
                            )
                            .child(ActivableRoute::new(
                                Route::Settings,
                                Link::new(Route::Settings)
                                    .child(SideBarItem::new().child("Settings")),
                            ))
                            .child(
                                SideBarItem::new()
                                    .on_press(|_| {
                                        println!("Pressed 🦀");
                                    })
                                    .child("Crab 🦀"),
                            ),
                    )
                    .content(rect().expanded().center().child(outlet::<Route>())),
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
