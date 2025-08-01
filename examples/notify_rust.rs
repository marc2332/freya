#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_router::prelude::*;

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
        Simple,
        #[route("/wow")]
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
                              }
                    }
                ),
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
    rsx!(
        label {
            "Simple -> /"
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
