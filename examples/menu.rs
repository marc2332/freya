#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Menus", (400.0, 350.0));
}

fn app() -> Element {
    let mut show_menu = use_signal(|| false);

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            padding: "16",
            Button {
                onclick: move |_| show_menu.toggle(),
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

#[allow(non_snake_case)]
#[component]
fn MenuItem(
    children: Element,

    theme: Option<ButtonThemeWith>,

    onclick: Option<EventHandler<Option<MouseEvent>>>,

    onmouseenter: Option<EventHandler<()>>,
) -> Element {
    let mut focus = use_focus();
    let mut status = use_signal(ButtonStatus::default);
    let platform = use_platform();

    let focus_id = focus.attribute();
    let click = &onclick;

    let ButtonTheme {
        hover_background,
        corner_radius,
        font_theme,
        ..
    } = use_applied_theme!(&theme, button);

    let onclick = {
        to_owned![click];
        move |ev| {
            focus.focus();
            if let Some(onclick) = &click {
                onclick.call(Some(ev))
            }
        }
    };

    use_drop(move || {
        if *status.read() == ButtonStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Pointer);
        status.set(ButtonStatus::Hovering);

        if let Some(onmouseenter) = &onmouseenter {
            onmouseenter.call(());
        }
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(ButtonStatus::default());
    };

    let background = match *status.read() {
        ButtonStatus::Hovering => &hover_background,
        ButtonStatus::Idle => "transparent",
    };

    rsx!(
        rect {
            onclick,
            onmouseenter,
            onmouseleave,
            focus_id,
            width: "fill-min",
            padding: "6",
            margin: "2",
            focusable: "true",
            role: "button",
            color: "{font_theme.color}",
            corner_radius: "{corner_radius}",
            background: "{background}",
            text_align: "start",
            main_align: "center",
            {children}
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn SubMenu(menu: Element, children: Element) -> Element {
    let my_id = use_context::<usize>();
    let mut menus = use_context::<Signal<Vec<usize>>>();
    let mut ids = use_context::<Signal<usize>>();
    let id = use_hook(|| {
        ids += 1;
        provide_context(*ids.peek()) // Use custom type
    });
    let show_menu = menus.read().contains(&id);

    rsx!(
        MenuItem {
            onmouseenter: move |_| {
                loop {
                    let last_menu_id = menus.read().last().cloned();
                    if let Some(last_menu_id) = last_menu_id {
                        if last_menu_id != my_id {
                            menus.write().pop();
                        } else {
                            break;
                        }
                    }else {
                        break;
                    }
                }

                let last_menu_id = menus.read().last().cloned();
                if let Some(last_menu_id) = last_menu_id {
                    if last_menu_id != id {
                        menus.write().push(id)
                    }
                } else {
                    menus.write().push(id)
                }
            },
            {children},
            if show_menu {
                rect {
                    position_top: "-12",
                    position_right: "-16",
                    position: "absolute",
                    width: "0",
                    height: "0",
                    rect {
                        width: "100v",
                        MenuContainer {
                            {menu}
                        }
                    }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn MenuButton(children: Element) -> Element {
    let mut menus = use_context::<Signal<Vec<usize>>>();
    let my_id = use_context::<usize>();
    rsx!(
        MenuItem {
            onmouseenter: move |_| {
                loop {
                    let last_menu_id = menus.read().last().cloned();
                    if let Some(last_menu_id) = last_menu_id {
                        if last_menu_id != my_id {
                            menus.write().pop();
                        } else {
                            break;
                        }
                    }else {
                        break;
                    }
                }
            },
            {children}
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn MenuContainer(children: Element) -> Element {
    rsx!(
        rect {
            background: "rgb(245, 245, 245)",
            corner_radius: "12",
            shadow: "0 4 5 0 rgb(0, 0, 0, 0.1)",
            padding: "4",
            content: "fit",
            {children}
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn Menu(children: Element, onclose: Option<EventHandler<()>>) -> Element {
    use_context_provider(|| Signal::new(0usize));
    use_context_provider::<Signal<Vec<usize>>>(|| Signal::new(vec![0]));
    provide_context(0usize);
    rsx!(
        rect {
            onglobalclick: move |_| {
                if let Some(onclose) = &onclose {
                    onclose.call(());
                }
            },
            MenuContainer {
                {children}
            }
        }
    )
}
