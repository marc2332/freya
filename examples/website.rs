#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "like freyaui.dev but in freya", (1500.0, 900.0));
}

fn app() -> Element {
    rsx!(
        rect {
            background: "rgb(24, 24, 27)",
            color: "white",
            font_family: "Inter",
            ThemeProvider {
                theme: DARK_THEME,
                ScrollView {
                    rect {
                        cross_align: "center",
                        width: "fill",
                        rect {
                            width: "60%",
                            spacing: "40",
                            padding: "40 0",
                            Navigation { }
                            Home { }
                        }
                    }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
fn Home() -> Element {
    rsx!(
        rect {
            cross_align: "center",
            width: "fill",
            spacing: "40",
            rect {
                direction: "horizontal",
                cross_align: "center",
                spacing: "12",
                BigFreyaLogo {  }
                rect {
                    spacing: "16",
                    label {
                        width: "500",
                        "Build native & cross-platform GUI applications using 🦀 Rust. Powered by 🧬 Dioxus and 🎨 Skia."
                    }
                    rect {
                        direction: "horizontal",
                        spacing: "10",
                        Link {
                            to: "https://book.freyaui.dev/getting_started.html",
                            Button {
                                theme: theme_with!(ButtonTheme {
                                    padding: "10 24".into(),
                                    border_fill: "none".into(),
                                    background: "rgb(14, 165, 233)".into(),
                                    hover_background: "rgb(2, 132, 199)".into(),
                                    font_theme: theme_with!(FontTheme {
                                        color: "black".into()
                                    }),
                                }),
                                label {
                                    "Get Started"
                                }
                            }
                        }
                        Link {
                            to: "https://github.com/marc2332/freya",
                            Button {
                                theme: theme_with!(ButtonTheme {
                                    padding: "10 24".into(),
                                    border_fill: "none".into(),
                                    background: "rgb(253, 186, 116)".into(),
                                    hover_background: "rgb(251, 146, 60)".into(),
                                    font_theme: theme_with!(FontTheme {
                                        color: "black".into()
                                    }),
                                }),
                                label {
                                    "Source Code"
                                }
                            }
                        }
                        Link {
                            to: "https://github.com/sponsors/marc2332",
                            Button {
                                theme: theme_with!(ButtonTheme {
                                    padding: "10 24".into(),
                                    border_fill: "none".into(),
                                    background: "rgb(249, 168, 212)".into(),
                                    hover_background: "rgb(244, 114, 182)".into(),
                                    font_theme: theme_with!(FontTheme {
                                        color: "black".into()
                                    }),
                                }),
                                label {
                                    "Sponsor"
                                }
                            }
                        }
                    }
                }
            }
            rect {
                width: "fill",
                cross_align: "center",
                rect {
                    background: "rgb(19, 19, 21)",
                    border: "1 solid rgb(41, 37, 36)",
                    corner_radius: "16",
                    width: "960",
                    height: "600",
                    spacing: "20",
                    padding: "20",
                    direction: "horizontal",
                    rect {
                        width: "calc(50% - 20)",
                        height: "fill"
                    }
                    rect {
                        width: "calc(50% - 20)",
                        height: "fill",
                        padding: "20",
                        spacing: "20",
                        rect {
                            height: "90%",
                            Counter {}
                        }
                        rect {
                            height: "fill",
                            width: "fill",
                            cross_align: "center",
                            Link {
                                to: "https://github.com/marc2332/freya#want-to-try-it-",
                                Button {
                                    theme: theme_with!(ButtonTheme {
                                        padding: "10 24".into(),
                                        border_fill: "none".into(),
                                        background: "rgb(109, 78, 233)".into(),
                                        hover_background: "rgb(87, 62, 186)".into(),
                                    }),
                                    label {
                                        "Run Locally"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}

import_svg!(IconLogo, "./freya_icon.svg", "50", "50");
import_svg!(FreyaLogo, "./freya_logo.svg", "50", "50");
import_svg!(BigFreyaLogo, "./freya_icon.svg", "150", "150");

#[allow(non_snake_case)]
fn Navigation() -> Element {
    rsx!(
        rect {
            direction: "horizontal",
            spacing: "24",
            cross_align: "center",
            color: "rgb(214, 211, 209)",
            IconLogo { }
            FreyaLogo { }
            Link {
                to: "https://freyaui.dev/blog",
                label {
                    "Book"
                }
            }
            Link {
                to: "https://book.freyaui.dev/",
                label {
                    "Book"
                }
            }
            Link {
                to: "https://docs.rs/freya/latest/freya/",
                label {
                    "Docs"
                }
            }
            Link {
                to: "https://discord.gg/sYejxCdewG",
                label {
                    "Discord"
                }
            }
        }
    )
}

#[allow(non_snake_case)]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);

    rsx!(
        ThemeProvider {
            theme: LIGHT_THEME,
            rect {
                corner_radius: "16",
                overflow: "clip",
                shadow: "0 0 10 0 rgb(0, 0, 0, 0.3)",
                rect {
                    height: "50%",
                    width: "100%",
                    main_align: "center",
                    cross_align: "center",
                    background: "rgb(0, 119, 182)",
                    color: "white",
                    shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
                    label {
                        font_size: "75",
                        font_weight: "bold",
                        "{count}"
                    }
                }
                rect {
                    height: "50%",
                    width: "100%",
                    main_align: "center",
                    cross_align: "center",
                    background: "white",
                    direction: "horizontal",
                    spacing: "8",
                    Button {
                        onclick: move |_| count += 1,
                        label { "Increase" }
                    }
                    Button {
                        onclick: move |_| count -= 1,
                        label { "Decrease" }
                    }
                }
            }
        }
    )
}
