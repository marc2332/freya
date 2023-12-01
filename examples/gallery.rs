#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_router::prelude::*;
use freya::prelude::*;

fn main() {
    launch_with_props(app, "Freya Gallery", (700.0, 500.0));
}

fn app(cx: Scope) -> Element {
    render!(Router::<Route> {})
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppSidebar)]
        #[route("/")]
        Home,
        #[route("/button")]
        ButtonDemo,
        #[route("/scrollview")]
        ScrollViewDemo,
        #[route("/dropdown")]
        DropdownDemo,
    #[end_layout]
    #[route("/..route")]
    PageNotFound { },
}

#[allow(non_snake_case)]
fn AppSidebar(cx: Scope) -> Element {
    render!(
        Sidebar {
            sidebar: render!(
                SidebarItem::<Route> {
                    to: Route::Home,
                    "Introduction"
                },
                SmallText {
                    "Components"
                }
                SidebarItem::<Route> {
                    to: Route::ButtonDemo,
                    "Button"
                },
                SidebarItem::<Route> {
                    to: Route::ScrollViewDemo,
                    "ScrollView"
                }
                SidebarItem::<Route> {
                    to: Route::DropdownDemo,
                    "Dropdown"
                }
            ),
            rect {
                width: "100%",
                height: "100%",
                padding: "20",
                Outlet::<Route> {  }
            }
        }
    )
}

#[allow(non_snake_case)]
fn Home(cx: Scope) -> Element {
    render!(
        label {
            font_size: "40",
            "Freya"
        }
        label {
            "Freya is a native GUI library for Rust. Powered by Dioxus and Skia."
        }
    )
}

#[allow(non_snake_case)]
fn ButtonDemo(cx: Scope) -> Element {
    let mut count = use_state(cx, || 4);

    render!(
        DemoTitle {
            "Button"
        }
        SectionTitle {
            "Example"
        }
        DemoBlock {
            Button {
                onclick: move |_| count += 1,
                label {
                    "{count}"
                }
            }
        }
    )
}

#[allow(non_snake_case)]
fn ScrollViewDemo(cx: Scope) -> Element {
    let show_scrollbar = use_state(cx, || true);

    render!(
        DemoTitle {
            "ScrollView"
        }
        SectionTitle {
            "Example"
        }
        DemoBlock {
            ScrollView {
                height: "200",
                width: "250",
                show_scrollbar: *show_scrollbar.get(),

                rect {
                    direction: "horizontal",
                    rect {
                        height: "80",
                        width: "120",
                        background: "rgb(236, 143, 94)"
                    }
                    rect {
                        height: "80",
                        width: "280",
                        background: "rgb(236, 227, 206)"
                    }
                }
                rect {
                    direction: "horizontal",
                    rect {
                        height: "70",
                        width: "190",
                        background: "rgb(243, 182, 100)"
                    }
                    rect {
                        height: "70",
                        width: "210",
                        background: "rgb(115, 144, 114)"
                    }
                }
                rect {
                    direction: "horizontal",
                    rect {
                        height: "60",
                        width: "260",
                        background: "rgb(241, 235, 144)"
                    }
                    rect {
                        height: "60",
                        width: "140",
                        background: "rgb(79, 111, 82)"
                    }
                }
                rect {
                    direction: "horizontal",
                    rect {
                        height: "50",
                        width: "330",
                        background: "rgb(159, 187, 115)"
                    }
                    rect {
                        height: "50",
                        width: "70",
                        background: "rgb(58, 77, 57)"
                    }
                }
            }
        }
        SmallTitle {
            "Show scrollbar?"
        }
        Switch {
            enabled: *show_scrollbar.get(),
            ontoggled: |_| show_scrollbar.set(!*show_scrollbar.get())
        }
    )
}

#[allow(non_snake_case)]
fn DropdownDemo(cx: Scope) -> Element {
    let values = cx.use_hook(|| vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    let selected_dropdown = use_state(cx, || "A".to_string());

    render!(
        DemoTitle {
            "Dropdown"
        }
        SectionTitle {
            "Example"
        }
        DemoBlock {
            Dropdown {
                value: selected_dropdown.get().clone(),
                values.iter().map(|ch| {
                    rsx!(
                        DropdownItem {
                            value: ch.to_string(),
                            onclick: move |_| selected_dropdown.set(ch.to_string()),
                            label { "{ch}" }
                        }
                    )
                })
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
fn DemoTitle<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    render!(label {
        font_size: "22",
        margin: "6 0 18 0",
        color: "rgb(20, 20, 20)",
        children
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn SectionTitle<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    render!(label {
        font_size: "18",
        margin: "4 0 12 0",
        color: "rgb(45, 45, 45)",
        children
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn SmallTitle<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    render!(label {
        font_size: "16",
        margin: "8 0 2 0",
        color: "rgb(25, 25, 25)",
        children
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn SmallText<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    render!(label {
        font_size: "14",
        margin: "4 0 2 0",
        color: "rgb(70, 70, 70)",
        children
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn DemoBlock<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    render!(rect {
        main_align: "center",
        cross_align: "center",
        width: "100%",
        padding: "30",
        background: "rgb(240, 240, 240)",
        corner_radius: "16",
        margin: "2 0",
        children
    })
}

#[allow(non_snake_case)]
fn PageNotFound(cx: Scope) -> Element {
    render!(
        label {
            "404!! 😵"
        }
    )
}
