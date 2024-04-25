use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;

use freya_hooks::{
    use_applied_theme, use_focus, use_platform, MenuContainerTheme, MenuContainerThemeWith,
    MenuItemTheme, MenuItemThemeWith,
};
use winit::window::CursorIcon;

/// Floating menu, use alongside [`MenuItem`].
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///    let mut show_menu = use_signal(|| false);
///
///    rsx!(
///        Body {
///            Button {
///                onclick: move |_| show_menu.toggle(),
///                label { "Open Menu" }
///            },
///            if *show_menu.read() {
///                Menu {
///                    onclose: move |_| show_menu.set(false),
///                    MenuButton {
///                        label {
///                            "Open"
///                        }
///                    }
///                    MenuButton {
///                        label {
///                            "Save"
///                        }
///                    }
///                    SubMenu {
///                        menu: rsx!(
///                            MenuButton {
///                                label {
///                                    "Some option"
///                                }
///                            }
///                        ),
///                        label {
///                            "Options"
///                        }
///                    }
///                    MenuButton {
///                        label {
///                            "Close"
///                        }
///                    }
///                }
///            }
///        }
///    )
///}
/// ```
#[allow(non_snake_case)]
#[component]
pub fn Menu(children: Element, onclose: Option<EventHandler<()>>) -> Element {
    // Provide the menus ID generator
    use_context_provider(|| Signal::new(ROOT_MENU.0));
    // Provide the menus stack
    use_context_provider::<Signal<Vec<MenuId>>>(|| Signal::new(vec![ROOT_MENU]));
    // Provide this the ROOT Menu ID
    use_context_provider(|| ROOT_MENU);

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

#[derive(Clone, Copy, PartialEq)]
struct MenuId(usize);

static ROOT_MENU: MenuId = MenuId(0);

fn close_menus_until(menus: &mut Signal<Vec<MenuId>>, until_to: MenuId) {
    loop {
        let last_menu_id = menus.read().last().cloned();
        if let Some(last_menu_id) = last_menu_id {
            if last_menu_id != until_to {
                menus.write().pop();
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

fn push_menu(menus: &mut Signal<Vec<MenuId>>, menu_id: MenuId) {
    let last_menu_id = menus.read().last().cloned();
    if let Some(last_menu_id) = last_menu_id {
        if last_menu_id != menu_id {
            menus.write().push(menu_id)
        }
    } else {
        menus.write().push(menu_id)
    }
}

/// Indicates the current status of the MenuItem.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum MenuItemStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the MenuItem.
    Hovering,
}

/// # Styling
/// Inherits the [`MenuItemTheme`](freya_hooks::MenuItemTheme) theme.
///
#[allow(non_snake_case)]
#[component]
pub fn MenuItem(
    /// Inner children for the MenuItem.
    children: Element,
    /// Theme override for the MenuItem.
    theme: Option<MenuItemThemeWith>,
    /// Handler for the `onclick` event.
    onclick: Option<EventHandler<Option<MouseEvent>>>,
    /// Handler for the `onmouseenter` event.
    onmouseenter: Option<EventHandler<()>>,
) -> Element {
    let mut focus = use_focus();
    let mut status = use_signal(MenuItemStatus::default);
    let platform = use_platform();

    let focus_id = focus.attribute();
    let click = &onclick;

    let MenuItemTheme {
        hover_background,
        corner_radius,
        font_theme,
    } = use_applied_theme!(&theme, menu_item);

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
        if *status.read() == MenuItemStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Pointer);
        status.set(MenuItemStatus::Hovering);

        if let Some(onmouseenter) = &onmouseenter {
            onmouseenter.call(());
        }
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(MenuItemStatus::default());
    };

    let background = match *status.read() {
        MenuItemStatus::Hovering => &hover_background,
        MenuItemStatus::Idle => "transparent",
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

/// Create sub menus inside a [`Menu`].
#[allow(non_snake_case)]
#[component]
pub fn SubMenu(
    /// Submenu configuration.
    menu: Element,
    /// Inner children for the MenuButton
    children: Element,
) -> Element {
    let parent_menu_id = use_context::<MenuId>();
    let mut menus = use_context::<Signal<Vec<MenuId>>>();
    let mut menus_ids_generator = use_context::<Signal<usize>>();
    let submenu_id = use_hook(|| {
        menus_ids_generator += 1;
        provide_context(MenuId(*menus_ids_generator.peek()))
    });

    let show_submenu = menus.read().contains(&submenu_id);

    rsx!(
        MenuItem {
            onmouseenter: move |_| {
                close_menus_until(&mut menus, parent_menu_id);
                push_menu(&mut menus, submenu_id);
            },
            {children},
            if show_submenu {
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

/// Like a button, but for [`Menu`]s.
#[allow(non_snake_case)]
#[component]
pub fn MenuButton(
    /// Inner children for the MenuButton
    children: Element,
    /// Handler for the `onclick` event.
    onclick: Option<EventHandler<Option<MouseEvent>>>,
) -> Element {
    let mut menus = use_context::<Signal<Vec<MenuId>>>();
    let parent_menu_id = use_context::<MenuId>();
    rsx!(
        MenuItem {
            onmouseenter: move |_| close_menus_until(&mut menus, parent_menu_id),
            onclick: move |e| {
                if let Some(onclick) = &onclick {
                    onclick.call(e)
                }
            }
            {children}
        }
    )
}

/// Wraps the body of a [`Menu`].
#[allow(non_snake_case)]
#[component]
pub fn MenuContainer(
    /// Inner children for the MenuContainer. Usually just `MenuButton` or `SubMenu`.
    children: Element,
    /// Theme override.
    theme: Option<MenuContainerThemeWith>,
) -> Element {
    let MenuContainerTheme {
        background,
        padding,
        shadow,
    } = use_applied_theme!(&theme, menu_container);
    rsx!(
        rect {
            background: "{background}",
            corner_radius: "12",
            shadow: "{shadow}",
            padding: "{padding}",
            content: "fit",
            {children}
        }
    )
}

#[cfg(test)]
mod test {
    use dioxus::prelude::use_signal;
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn menu() {
        fn menu_app() -> Element {
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
                                                    "Option 3"
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

        let mut utils = launch_test(menu_app);
        utils.wait_for_update().await;

        let start_size = utils.sdom().get().layout().size();

        assert_eq!(utils.sdom().get().layout().size(), 5);

        // Open the Menu
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (5.0, 5.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        // Check the `Open` button exists
        assert_eq!(
            utils
                .root()
                .get(0)
                .get(1)
                .get(0)
                .get(0)
                .get(0)
                .get(0)
                .text(),
            Some("Open")
        );

        assert!(utils.sdom().get().layout().size() > start_size);

        // Close the Menu
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (15.0, 60.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        assert_eq!(utils.sdom().get().layout().size(), start_size);

        // Open the Menu again
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (15.0, 15.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        let one_submenu_opened = utils.sdom().get().layout().size();
        assert!(one_submenu_opened > start_size);

        // Open the SubMenu
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseOver,
            cursor: (15.0, 130.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        // Check the `Option 1` button exists
        assert_eq!(
            utils
                .root()
                .get(0)
                .get(1)
                .get(0)
                .get(2)
                .get(1)
                .get(0)
                .get(0)
                .get(0)
                .get(0)
                .get(0)
                .text(),
            Some("Option 1")
        );

        assert!(utils.sdom().get().layout().size() > one_submenu_opened);

        // Stop showing the submenu
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseOver,
            cursor: (15.0, 90.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        assert_eq!(utils.sdom().get().layout().size(), one_submenu_opened);

        // Click somewhere also so all the menus hide
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (333.0, 333.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        assert_eq!(utils.sdom().get().layout().size(), start_size);
    }
}
