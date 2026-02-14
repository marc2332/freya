#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    prelude::*,
    router::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_title("Complex Router")))
}

fn app() -> impl IntoElement {
    Router::<Route>::new(|| RouterConfig::default().with_initial_path(Route::Home))
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home,
        #[route("/about")]
        About,
        #[nest("/users/:user_id")]
            #[layout(UserLayout)]
                #[route("/")]
                UserDetail { user_id: String },
                #[route("/posts")]
                UserPosts { user_id: String },
}

#[derive(PartialEq)]
struct AppLayout;
impl Component for AppLayout {
    fn render(&self) -> impl IntoElement {
        NativeRouter::new().child(
            rect()
                .content(Content::flex())
                .child(
                    rect()
                        .horizontal()
                        .width(Size::fill())
                        .height(Size::px(50.))
                        .background((230, 230, 230))
                        .padding(12.)
                        .spacing(12.)
                        .cross_align(Alignment::center())
                        .content(Content::Flex)
                        .child(
                            ActivableRoute::new(
                                Route::Home,
                                Link::new(Route::Home).child(Button::new().flat().child("Home")),
                            )
                            .exact(true),
                        )
                        .child(
                            ActivableRoute::new(
                                Route::About,
                                Link::new(Route::About).child(Button::new().flat().child("About")),
                            )
                            .exact(true),
                        )
                        .child(rect().width(Size::flex(1.)))
                        .child(
                            Button::new()
                                .flat()
                                .on_press(|_| {
                                    RouterContext::get().go_back();
                                })
                                .child("Go Back"),
                        ),
                )
                .child(
                    rect()
                        .expanded()
                        .background((240, 240, 240))
                        .padding(12.)
                        .child(Outlet::<Route>::new()),
                ),
        )
    }
}

#[derive(PartialEq)]
struct UserLayout {
    user_id: String,
}
impl Component for UserLayout {
    fn render(&self) -> impl IntoElement {
        let user_id = self.user_id.clone();
        rect()
            .spacing(6.)
            .child(
                rect()
                    .background((200, 220, 240))
                    .corner_radius(8.)
                    .padding(4.)
                    .child(format!("User: {}", user_id)),
            )
            .child(
                rect()
                    .horizontal()
                    .spacing(6.)
                    .child(
                        ActivableRoute::new(
                            Route::UserDetail {
                                user_id: user_id.clone(),
                            },
                            Link::new(Route::UserDetail {
                                user_id: user_id.clone(),
                            })
                            .child(Button::new().rounded_full().child("Details")),
                        )
                        .exact(true),
                    )
                    .child(
                        ActivableRoute::new(
                            Route::UserPosts {
                                user_id: user_id.clone(),
                            },
                            Link::new(Route::UserPosts {
                                user_id: user_id.clone(),
                            })
                            .child(Button::new().rounded_full().child("Posts")),
                        )
                        .exact(true),
                    ),
            )
            .child(rect().padding((0., 6.)).child(Outlet::<Route>::new()))
    }
}

#[derive(PartialEq)]
struct Home;
impl Component for Home {
    fn render(&self) -> impl IntoElement {
        rect().child(
            rect()
                .horizontal()
                .spacing(8.)
                .child(
                    Button::new()
                        .on_press(|_| {
                            RouterContext::get().push(Route::UserDetail {
                                user_id: "alice".to_string(),
                            });
                        })
                        .child("Visit Alice"),
                )
                .child(
                    Button::new()
                        .on_press(|_| {
                            RouterContext::get().push(Route::UserDetail {
                                user_id: "bob".to_string(),
                            });
                        })
                        .child("Visit Bob"),
                ),
        )
    }
}

#[derive(PartialEq)]
struct About;
impl Component for About {
    fn render(&self) -> impl IntoElement {
        rect().child(label().font_size(24.).text("About")).child(
            Button::new()
                .on_press(|_| {
                    RouterContext::get().replace(Route::Home);
                })
                .child("Replace with Home"),
        )
    }
}

#[derive(PartialEq)]
struct UserDetail {
    user_id: String,
}
impl Component for UserDetail {
    fn render(&self) -> impl IntoElement {
        let user_id = self.user_id.clone();
        rect()
            .spacing(6.)
            .child(
                label()
                    .font_size(20.)
                    .text(format!("User Details for {}", user_id)),
            )
            .child(format!("Username: {}", user_id))
            .child(format!("Email: {}@example.com", user_id))
    }
}

#[derive(PartialEq)]
struct UserPosts {
    user_id: String,
}
impl Component for UserPosts {
    fn render(&self) -> impl IntoElement {
        let user_id = self.user_id.clone();
        rect()
            .spacing(6.)
            .child(label().font_size(20.).text(format!("Posts by {}", user_id)))
            .child(
                rect()
                    .width(Size::fill())
                    .background((220, 230, 240))
                    .corner_radius(4.)
                    .padding(12.)
                    .child(
                        rect()
                            .child("Post 1: First Post")
                            .child(rect().padding((0., 4.)).child("This is my first post!")),
                    ),
            )
            .child(
                rect()
                    .width(Size::fill())
                    .background((220, 230, 240))
                    .corner_radius(4.)
                    .padding(12.)
                    .child(
                        rect()
                            .child("Post 2: Second Post")
                            .child(rect().padding((0., 4.)).child("This is my second post!")),
                    ),
            )
            .child(
                rect()
                    .width(Size::fill())
                    .background((220, 230, 240))
                    .corner_radius(4.)
                    .padding(12.)
                    .child(
                        rect()
                            .child("Post 3: Third Post")
                            .child(rect().padding((0., 4.)).child("This is my third post!")),
                    ),
            )
    }
}
