use freya::prelude::*;

#[allow(non_snake_case)]
pub fn DsMenu() -> Element {
    let mut show_menu = use_signal(|| false);

    rsx!(
        Body {
            Button {
                onclick: move |_| show_menu.toggle(),
                label { "Open Menu" }
            },
            if *show_menu.read() {
                Menu {
                    onclose: move |_| show_menu.set(false),
                    MenuItem {
                        label {
                            "MenuItem"
                        }
                    }
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
                                    "Some option"
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
