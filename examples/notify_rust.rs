#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::thread;

use freya::prelude::*;
use freya_router::prelude::*;
use notify_rust::{
    Hint,
    Notification,
};

fn main() {
    launch_with_props(app, "Notify Example", (550.0, 400.0));
}

fn app() -> Element {
    rsx!(Router::<Route> {})
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppSidebar)]
        #[route("/")]
        Simple,
        #[route("/simple-async")]
        SimpleAsync,
        #[route("/minimal")]
        Minimal,
        #[route("/actions")]
        Actions,
        #[route("/on-close")]
        OnClose,
      #[route("/on-close-reason")]
        OnCloseReason,
    #[end_layout]
    #[route("/..route")]
    PageNotFound { },
}

#[allow(non_snake_case)]
fn AppSidebar() -> Element {
    let PlatformInformation { viewport_size, .. } = *use_platform_information().read();
    let variable_width: &str;
    if viewport_size.width > 640.0 && viewport_size.width < 1024.0 {
        variable_width = "40%".into();
    } else if viewport_size.width >= 1024.0 {
        variable_width = "30%";
    } else {
        variable_width = "50%";
    }

    rsx!(
        NativeRouter {
            Sidebar {
                width: "{variable_width}",
                sidebar: rsx!(
                    Accordion {
                        summary: rsx!(
                            AccordionSummary {
                                label {
                                    "Basic Notifications"
                                }
                            }),
                            AccordionBody {
                                Link {
                                    to: Route::Simple,
                                    ActivableRoute {
                                        route: Route::Simple,
                                        exact: true,
                                        SidebarItem {
                                            theme: SidebarItemThemeWith {
                                            corner_radius: Some(Cow::Borrowed("6")),
                                            ..Default::default()
                                            },
                                            label {
                                                "Simple Notification"
                                            }
                                        }
                                  }
                                }
                                Link {
                                      to: Route::SimpleAsync,
                                      ActivableRoute {
                                          route: Route::SimpleAsync,
                                          exact: true,
                                          SidebarItem {
                                              theme: SidebarItemThemeWith {
                                                  corner_radius: Some(Cow::Borrowed("6")),
                                                  ..Default::default()
                                              },
                                              label {
                                                  "Simple (Asynchronous)"
                                              }
                                          }
                                      }
                                }
                                Link {
                                    to: Route::Minimal,
                                    ActivableRoute {
                                        route: Route::Minimal,
                                        exact: true,
                                        SidebarItem {
                                            theme: SidebarItemThemeWith {
                                            corner_radius: Some(Cow::Borrowed("6")),
                                            ..Default::default()
                                            },
                                            label {
                                                "Minimal Notification"
                                            }
                                        }
                                    }
                                }

                            }
                    },
                    Accordion {
                        summary:
                            rsx!(
                                AccordionSummary {
                                    label {
                                        "Action Notifications"
                                    }
                                }),
                                AccordionBody {
                                    Link {
                                        to: Route::Actions,
                                        ActivableRoute {
                                            route: Route::Actions,
                                            exact: true,
                                            SidebarItem {
                                                theme: SidebarItemThemeWith {
                                                corner_radius: Some(Cow::Borrowed("6")),
                                                ..Default::default()
                                                },
                                                label {
                                                    "Notification With Action"
                                                }
                                            }
                                        }
                                    }
                                    Link {
                                        to: Route::OnClose,
                                        ActivableRoute {
                                            route: Route::OnClose,
                                            exact: true,
                                            SidebarItem {
                                                theme: SidebarItemThemeWith {
                                                corner_radius: Some(Cow::Borrowed("6")),
                                                ..Default::default()
                                                },
                                                label {
                                                    "Notify On Close With Message"
                                                }
                                            }
                                        }
                                    }
                                    Link {
                                        to: Route::OnCloseReason,
                                        ActivableRoute {
                                            route: Route::OnCloseReason,
                                            exact: true,
                                            SidebarItem {
                                                theme: SidebarItemThemeWith {
                                                corner_radius: Some(Cow::Borrowed("6")),
                                                ..Default::default()
                                                },
                                                label {
                                                    "Notify On Close With Reason"
                                                }
                                            }
                                        }
                                    }
                                }
                    }),
                Body {
                    main_align: "center",
                    cross_align: "center",
                    width: "100%",
                    height: "100%",
                    Outlet::<Route> {  }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn Simple() -> Element {
    let PlatformInformation { viewport_size, .. } = *use_platform_information().read();
    let variable_width: &str;

    if viewport_size.width > 640.0 && viewport_size.width < 1024.0 {
        variable_width = "70%";
    } else if viewport_size.width >= 1024.0 {
        variable_width = "60%";
    } else {
        variable_width = "90%";
    }

    fn notify() {
        let _ = Notification::new()
            .summary("Firefox News")
            .body("This will almost look like a real firefox notification.")
            .icon("firefox")
            .show();
    }

    rsx!(
        Button {
            theme: ButtonThemeWith {
                padding: Some(Cow::Borrowed("16 8")),
         width: Some(Cow::Borrowed(variable_width)),
         ..Default::default()
            },
            onclick: |_| notify(),
         label {
             "Notify"
         }
        }
    )
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
#[allow(non_snake_case)]
#[component]
fn SimpleAsync() -> Element {
  rsx!(
    rect {
      label { "This is an xdg only feature" }
    }
  )
}

#[cfg(all(unix, not(target_os = "macos")))]
#[allow(non_snake_case)]
#[component]
fn SimpleAsync() -> Element {
    let PlatformInformation { viewport_size, .. } = *use_platform_information().read();
    let variable_width: &str;

    if viewport_size.width > 640.0 && viewport_size.width < 1024.0 {
        variable_width = "70%";
    } else if viewport_size.width >= 1024.0 {
        variable_width = "60%";
    } else {
        variable_width = "90%";
    }

    async fn notify_async() {
        let _ = Notification::new()
            .summary("async notification")
            .body("this notification was sent via an async api")
            .icon("dialog-positive")
            .show_async()
            .await;
    }
    rsx!(
        Button {
            theme: ButtonThemeWith {
                padding: Some(Cow::Borrowed("16 8")),
         width: Some(Cow::Borrowed(variable_width)),
         ..Default::default()
            },
            onclick: |_| notify_async(),
         label {
             "Notify Asychronously"
         }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn Minimal() -> Element {
    let PlatformInformation { viewport_size, .. } = *use_platform_information().read();
    let variable_width: &str;

    if viewport_size.width > 640.0 && viewport_size.width < 1024.0 {
        variable_width = "70%";
    } else if viewport_size.width >= 1024.0 {
        variable_width = "60%";
    } else {
        variable_width = "90%";
    }

    fn notify_minimal() {
        let _ = Notification::new().summary("minimal notification").show();
    }

    rsx!(
        Button {
            theme: ButtonThemeWith {
                padding: Some(Cow::Borrowed("16 8")),
         width: Some(Cow::Borrowed(variable_width)),
         ..Default::default()
            },
            onclick: |_| notify_minimal(),
         label {
             "Notify Minimal"
         }
        }
    )
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
#[allow(non_snake_case)]
#[component]
fn Actions() -> Element {
  rsx!(
    rect {
      label { "This is an xdg only feature" }
    }
  )
}

#[cfg(all(unix, not(target_os = "macos")))]
#[allow(non_snake_case)]
#[component]
fn Actions() -> Element {
    let PlatformInformation { viewport_size, .. } = *use_platform_information().read();
    let variable_width: &str;

    if viewport_size.width > 640.0 && viewport_size.width < 1024.0 {
        variable_width = "70%";
    } else if viewport_size.width >= 1024.0 {
        variable_width = "60%";
    } else {
        variable_width = "90%";
    }

    fn actions() {
        Notification::new()
            .summary("click me")
            .action("default", "default") // IDENTIFIER, LABEL
            .action("clicked_a", "button a") // IDENTIFIER, LABEL
            .action("clicked_b", "button b") // IDENTIFIER, LABEL
            .hint(Hint::Resident(true))
            .show()
            .unwrap()
            .wait_for_action(|action| match action {
                "default" => println!("default"),
                "clicked_a" => println!("clicked a"),
                "clicked_b" => println!("clicked b"),
                // FIXME: here "__closed" is a hardcoded keyword, it will be deprecated!!
                "__closed" => println!("the notification was closed"),
                _ => (),
            });
    }
    rsx!(
        Button {
            theme: ButtonThemeWith {
                padding: Some(Cow::Borrowed("16 8")),
         width: Some(Cow::Borrowed(variable_width)),
         ..Default::default()
            },
            onclick: |_| actions(),
         label {
             "Notify Actions"
         }
        }
    )
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
#[allow(non_snake_case)]
#[component]
fn OnClose() -> Element {
  rsx!(
    rect {
      label { "This is an xdg only feature" }
    }
  )
}

#[cfg(all(unix, not(target_os = "macos")))]
#[allow(non_snake_case)]
#[component]
fn OnClose() -> Element {
    let PlatformInformation { viewport_size, .. } = *use_platform_information().read();
    let variable_width: &str;

    if viewport_size.width > 640.0 && viewport_size.width < 1024.0 {
        variable_width = "70%";
    } else if viewport_size.width >= 1024.0 {
        variable_width = "60%";
    } else {
        variable_width = "90%";
    }

    fn notify_on_close() {
        thread::spawn(|| {
            let _ = Notification::new()
                .summary("Time is running out")
                .body("This will go away.")
                .icon("clock")
                .show()
                .map(|handler| handler.on_close(|| println!("Notification closed")));
        });
    }

    rsx!(
        Button {
            theme: ButtonThemeWith {
                padding: Some(Cow::Borrowed("16 8")),
         width: Some(Cow::Borrowed(variable_width)),
         ..Default::default()
            },
            onclick: move |_| notify_on_close(),
         label {
             "Notify With On Close Message"
         }
        }
    )
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
#[allow(non_snake_case)]
#[component]
fn OnCloseReason() -> Element {
  rsx!(
    rect {
      label { "This is an xdg only feature" }
    }
  )
}

#[cfg(all(unix, not(target_os = "macos")))]
#[allow(non_snake_case)]
#[component]
fn OnCloseReason() -> Element {
    let PlatformInformation { viewport_size, .. } = *use_platform_information().read();
    let variable_width: &str;

    if viewport_size.width > 640.0 && viewport_size.width < 1024.0 {
        variable_width = "70%";
    } else if viewport_size.width >= 1024.0 {
        variable_width = "60%";
    } else {
        variable_width = "90%";
    }

    fn notify_on_close_reason() {
        thread::spawn(|| {
            let _ = Notification::new()
                .summary("Time is running out")
                .body("This will go away.")
                .icon("clock")
                .show()
                .map(|handler| {
                    handler.on_close(|reason| println!("Close due to reason: \"{reason:?}\""))
                });
        });
    }

    rsx!(
        Button {
            theme: ButtonThemeWith {
                padding: Some(Cow::Borrowed("16 8")),
         width: Some(Cow::Borrowed(variable_width)),
         ..Default::default()
            },
            onclick: move |_| notify_on_close_reason(),
         label {
             "Notify With On Close Reason"
         }
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
