#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{common::NodeReferenceLayout, prelude::*};

fn main() {
    launch_with_props(app, "Menus", (400.0, 350.0));
}

fn app() -> Element {
    let mut show_menu = use_signal(|| false);

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            Button {
                onclick: move |_| show_menu.toggle(),
                label { "Show/Hide" }
            },
            if *show_menu.read() {
                Menu {
                    width: "100",
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
                        width: "125",
                        menu: rsx!(
                            MenuButton {
                                label {
                                    "Option 1"
                                }
                            }
                            SubMenu {
                                width: "100",
                                menu: rsx!(
                                    MenuButton {
                                        label {
                                            "Option 3"
                                        }
                                    }
                                    MenuButton {
                                        label {
                                            "Option 4"
                                        }
                                    }
                                ),
                                label {
                                    "More Options"
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

#[component]
fn MenuItem(
    children: Element,
    theme: Option<ButtonThemeWith>,
    onclick: Option<EventHandler<Option<MouseEvent>>>,
) -> Element {
    let mut focus = use_focus();
    let mut status = use_signal(ButtonStatus::default);
    let platform = use_platform();

    let focus_id = focus.attribute();
    let click = &onclick;

    let ButtonTheme {
        background,
        hover_background,
        border_fill,
        focus_border_fill,
        padding,
        margin,
        corner_radius,
        height,
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
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(ButtonStatus::default());
    };

    let background = match *status.read() {
        ButtonStatus::Hovering => &hover_background,
        ButtonStatus::Idle => "transparent",
    };
    let border = if focus.is_selected() {
        format!("2 solid {focus_border_fill}")
    } else {
        format!("1 solid {border_fill}")
    };

    rsx!(
        rect {
            onclick,
            onmouseenter,
            onmouseleave,
            focus_id,
            width: "100%",
            padding: "6",
            margin: "2",
            focusable: "true",
            overflow: "clip",
            role: "button",
            color: "{font_theme.color}",
            corner_radius: "{corner_radius}",
            background: "{background}",
            text_align: "start",
            main_align: "center",
            cross_align: "center",
            {children}
        }
    )
}

#[component]
fn SubMenu(menu: Element, children: Element, width: String) -> Element {
    let mut menus = use_context::<Signal<Vec<usize>>>();
    let mut ids = use_context::<Signal<usize>>();
    let id = use_hook(|| {
        ids += 1;
        *ids.peek()
    });
    provide_context(id); // Use custom type
    let mut show_menu = menus.read().contains(&id);
    let container_size = consume_context::<Signal<NodeReferenceLayout>>();
    let submenus_offsets = container_size.read().area.width() - 2.;

    rsx!(
        rect {
            onmouseenter: move |_| {
                let last_menu_id = menus.read().last().cloned();
                if let Some(last_menu_id) = last_menu_id {
                    if last_menu_id != id {
                        menus.write().push(id)
                    }
                } else {
                    menus.write().push(id)
                }
            },
            MenuItem {
                {children},
            },
            if show_menu {
                rect {
                    position_left: "{submenus_offsets}",
                    position: "absolute",
                    MenuContainer {
                        width,
                        {menu}
                    }
                }
            }
        }
    )
}

#[component]
fn MenuButton(children: Element) -> Element {
    let mut menus = use_context::<Signal<Vec<usize>>>();
    let my_id = consume_context::<usize>();
    rsx!(
        rect {
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
            onmouseleave: |_| {},
            MenuItem {
                {children}
            }
        }
    )
}

#[component]
fn MenuContainer(children: Element, width: String) -> Element {
    let mut menus = use_context::<Signal<Vec<usize>>>();
    let (reference, size) = use_node_signal();
    provide_context(size); // Use custom type
    rsx!(
        rect {
            reference,
            background: "rgb(245, 245, 245)",
            corner_radius: "12",
            width,
            shadow: "0 4 5 0 rgb(0, 0, 0, 0.1)",
            padding: "4",
            {children}
        }
    )
}

#[component]
fn Menu(children: Element, onclose: Option<EventHandler<()>>, width: String) -> Element {
    use_context_provider(|| Signal::new(0usize));
    let menus = use_context_provider::<Signal<Vec<usize>>>(|| Signal::new(vec![0]));
    provide_context(0usize);
    rsx!(
        rect {
            onglobalclick: move |_| {
                if let Some(onclose) = &onclose {
                    onclose.call(());
                }
            },
            MenuContainer {
                width,
                {children}
            }
        }
    )
}
