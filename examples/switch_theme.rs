#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

#[component]
fn ThemeChanger() -> Element {
    let mut theme = use_theme();

    rsx!(
        Tile {
            onselect: move |_| theme.set(BANANA_THEME),
            leading: rsx!(
                Radio {
                    selected: theme.read().name == "banana",
                },
            ),
            label { "Banana" }
        }
        Tile {
            onselect: move |_| theme.set(DARK_THEME),
            leading: rsx!(
                Radio {
                    selected: theme.read().name == "dark",
                },
            ),
            label { "Dark" }
        }
        Tile {
            onselect: move |_| theme.set(LIGHT_THEME),
            leading: rsx!(
                Radio {
                    selected: theme.read().name == "light",
                },
            ),
            label { "Light" }
        }
    )
}

fn app() -> Element {
    use_init_default_theme();
    let mut value = use_signal::<f64>(|| 50.);

    rsx!(
        Body {
            ScrollView {
                rect {
                    spacing: "20",
                    padding: "40",
                    cross_align: "center",
                    Switch {
                        enabled: value() >= 50.,
                        ontoggled: move |_| {
                            if value() >= 50. {
                                value.set(25.0);
                            } else {
                                value.set(75.0);
                            }
                        }
                    }
                    Slider {
                        width: "fill",
                        value: value(),
                        onmoved: move |e| value.set(e),
                    }
                    ProgressBar {
                        show_progress: true,
                        progress: value() as f32
                    }
                    Tile {
                        onselect: move |_| {
                            if value() >= 50. {
                                value.set(25.0);
                            } else {
                                value.set(75.0);
                            }
                        },
                        leading: rsx!(
                            Checkbox {
                                selected: value() >= 50.,
                            },
                        ),
                        label { "First choice" }
                    }
                    Tile {
                        onselect: move |_| {
                            if value() < 50. {
                                value.set(75.0);
                            } else {
                                value.set(25.0);
                            }
                        },
                        leading: rsx!(
                            Checkbox {
                                selected: value() < 50.,
                            },
                        ),
                        label { "Second choice" }
                    }
                    Input {
                        value: value().round().to_string(),
                        onchange: move |num: String| {
                            if let Ok(num) = num.parse() {
                                *value.write() = num;
                            }
                        }
                    }
                    Button {
                        onclick: move |_| value.set(35.),
                        label {
                            "Set to 35%"
                        }
                    }
                    ThemeChanger { }
                }
            }
            SnackBar {
                show: value() >= 50.,
                label {
                    "Hello!"
                }
            }
        }
    )
}
