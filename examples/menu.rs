#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Menus", (550.0, 450.0));
}

fn app() -> Element {
    use_init_theme(|| DARK_THEME);
    let mut show_menu = use_signal(|| false);

    rsx!(
        Body {
            theme: theme_with!(BodyTheme {
                padding: "20".into()
            }),
            Button {
                onpress: move |_| show_menu.toggle(),
                label { "Open Menu" }
            },
            if *show_menu.read() {
                Menu {
                    onclose: move |_| show_menu.set(false),
                    MenuButton {
                        label {
                            "Open"
                        }
                    }
                    MenuButton {
                        label {
                            "Save"
                        }
                    }
                    SubMenu {
                        menu: rsx!(
                            MenuButton {
                                label {
                                    "Option 1"
                                }
                            }
                            SubMenu {
                                menu: rsx!(
                                    MenuButton {
                                        label {
                                            "Option 2"
                                        }
                                    }
                                    MenuButton {
                                        label {
                                            "Option 3"
                                        }
                                    }
                                ),
                                label {
                                    "More Options"
                                }
                            }
                            SubMenu {
                                menu: rsx!(
                                    MenuButton {
                                        onpress: |_| println!("clicked option 4"),
                                        label {
                                            "Option 4"
                                        }
                                    }
                                    MenuButton {
                                        label {
                                            "Option 5"
                                        }
                                    }
                                    SubMenu {
                                        menu: rsx!(
                                            MenuButton {
                                                label {
                                                    "Option 6"
                                                }
                                            }
                                            MenuButton {
                                                onpress: |_| println!("clicked option 7"),
                                                label {
                                                    "Option 7"
                                                }
                                            }
                                            MenuButton {
                                                label {
                                                    "Option 8"
                                                }
                                            }
                                        ),
                                        label {
                                            "More Options"
                                        }
                                    }
                                ),
                                label {
                                    "Even More Options"
                                }
                            }
                        ),
                        label {
                            "Options"
                        }
                    }
                    MenuButton {
                        label {
                            "Close"
                        }
                    }
                }
            }
        }
    )
}
