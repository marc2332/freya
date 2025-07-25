use dioxus::prelude::*;
use freya_core::{
    platform::CursorIcon,
    types::AccessibilityId,
};
use freya_elements::{
    self as dioxus_elements,
    events::{
        keyboard::Key,
        KeyboardEvent,
        MouseEvent,
    },
};
use freya_hooks::{
    theme_with,
    use_applied_theme,
    use_focus,
    use_platform,
    DropdownItemTheme,
    DropdownItemThemeWith,
    DropdownTheme,
    DropdownThemeWith,
    IconThemeWith,
    UseFocus,
};

use crate::icons::ArrowIcon;

/// Properties for the [`DropdownItem`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DropdownItemProps {
    /// Theme override.
    pub theme: Option<DropdownItemThemeWith>,
    /// Selectable items, like [`DropdownItem`]
    pub children: Element,
    /// Handler for the `onpress` event.
    pub onpress: Option<EventHandler<()>>,
    /// Render this item as selected.
    #[props(default = false)]
    pub selected: bool,
}

/// Current status of the DropdownItem.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DropdownItemStatus {
    /// Default state.
    #[default]
    Idle,
    /// Dropdown item is being hovered.
    Hovering,
}

/// # Styling
/// Inherits the [`DropdownItemTheme`](freya_hooks::DropdownItemTheme) theme.
#[allow(non_snake_case)]
pub fn DropdownItem(
    DropdownItemProps {
        theme,
        children,
        onpress,
        selected,
    }: DropdownItemProps,
) -> Element {
    let theme = use_applied_theme!(&theme, dropdown_item);
    let focus = use_focus();
    let mut status = use_signal(DropdownItemStatus::default);
    let platform = use_platform();
    let dropdown_group = use_context::<DropdownGroup>();

    let a11y_id = focus.attribute();
    let a11y_member_of = UseFocus::attribute_for_id(dropdown_group.group_id);

    let DropdownItemTheme {
        font_theme,
        background,
        hover_background,
        select_background,
        border_fill,
        select_border_fill,
    } = &theme;

    let background = match *status.read() {
        _ if selected => select_background,
        DropdownItemStatus::Hovering => hover_background,
        DropdownItemStatus::Idle => background,
    };
    let border = if focus.is_focused_with_keyboard() {
        format!("2 inner {select_border_fill}")
    } else {
        format!("1 inner {border_fill}")
    };

    use_drop(move || {
        if *status.peek() == DropdownItemStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onpointerenter = move |_| {
        platform.set_cursor(CursorIcon::Pointer);
        status.set(DropdownItemStatus::Hovering);
    };

    let onpointerleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(DropdownItemStatus::default());
    };

    let onkeydown = {
        to_owned![onpress];
        move |ev: KeyboardEvent| {
            if ev.key == Key::Enter {
                if let Some(onpress) = &onpress {
                    onpress.call(())
                }
            }
        }
    };

    let onclick = move |_: MouseEvent| {
        if let Some(onpress) = &onpress {
            onpress.call(())
        }
    };

    rsx!(
        rect {
            width: "fill-min",
            color: "{font_theme.color}",
            a11y_id,
            a11y_role: "button",
            a11y_member_of,
            background: "{background}",
            border,
            padding: "6 10",
            corner_radius: "6",
            main_align: "center",
            onpointerenter,
            onpointerleave,
            onclick,
            onkeydown,
            {children}
        }
    )
}

/// Properties for the [`Dropdown`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps {
    /// Theme override.
    pub theme: Option<DropdownThemeWith>,
    /// The selected item element.
    pub selected_item: Element,
    /// Selectable items elements, like [`DropdownItem`].
    pub children: Element,
}

/// Current status of the Dropdown.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DropdownStatus {
    /// Default state.
    #[default]
    Idle,
    /// Dropdown is being hovered.
    Hovering,
}

#[derive(Clone)]
struct DropdownGroup {
    group_id: AccessibilityId,
}

/// Select from multiple options, use alongside [`DropdownItem`].
///
/// # Styling
/// Inherits the [`DropdownTheme`](freya_hooks::DropdownTheme) theme.
///
/// # Example
/// ```rust
/// # use freya::prelude::*;
///
/// fn app() -> Element {
///     let values = use_signal(|| vec!["Value A".to_string(), "Value B".to_string(), "Value C".to_string()]);
///     let mut selected_dropdown = use_signal(|| 0);
///     rsx!(
///         Dropdown {
///             selected_item: rsx!( label { "{values.read()[selected_dropdown()]}" } ),
///             for (i, ch) in values.iter().enumerate() {
///                 DropdownItem {
///                     selected: selected_dropdown() == i,
///                     onpress: move |_| selected_dropdown.set(i),
///                     label { "{ch}" }
///                 }
///             }
///         }
///     )
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rsx!(
/// #       Preview {
/// #           {app()}
/// #       }
/// #   )
/// # }, (250., 250.).into(), "./images/gallery_dropdown.png");
/// ```
///
/// # Preview
/// ![Dropdown Preview][dropdown]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("dropdown", "images/gallery_dropdown.png")
)]
#[allow(non_snake_case)]
pub fn Dropdown(
    DropdownProps {
        selected_item,
        theme,
        children,
    }: DropdownProps,
) -> Element {
    let theme = use_applied_theme!(&theme, dropdown);
    let mut focus = use_focus();
    let mut status = use_signal(DropdownStatus::default);
    let mut opened = use_signal(|| false);
    let platform = use_platform();

    use_context_provider(|| DropdownGroup {
        group_id: focus.id(),
    });

    let a11y_id = focus.attribute();
    let a11y_member_of = focus.attribute();

    // Close if the focused node is not part of the Dropdown
    use_effect(move || {
        if let Some(member_of) = focus.focused_node().read().member_of() {
            if member_of != focus.id() {
                opened.set(false);
            }
        }
    });

    use_drop(move || {
        if *status.peek() == DropdownStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    // Close the dropdown if clicked anywhere
    let onglobalpointerup = move |_| {
        opened.set(false);
    };

    let onclick = move |_| {
        focus.request_focus();
        opened.toggle();
    };

    let onglobalkeydown = move |ev: KeyboardEvent| {
        // Close when `Escape` key is pressed
        if ev.key == Key::Escape {
            opened.set(false);
        }
    };

    let onkeydown = move |ev: KeyboardEvent| {
        // Open the dropdown items when the `Enter` key is pressed
        if ev.key == Key::Enter {
            opened.toggle();
        }
    };

    let onpointerenter = move |_| {
        platform.set_cursor(CursorIcon::Pointer);
        status.set(DropdownStatus::Hovering);
    };

    let onpointerleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(DropdownStatus::default());
    };

    let DropdownTheme {
        width,
        margin,
        font_theme,
        dropdown_background,
        background_button,
        hover_background,
        border_fill,
        focus_border_fill,
        arrow_fill,
    } = &theme;

    let background = match *status.read() {
        DropdownStatus::Hovering => hover_background,
        DropdownStatus::Idle => background_button,
    };
    let border = if focus.is_focused_with_keyboard() {
        format!("2 inner {focus_border_fill}")
    } else {
        format!("1 inner {border_fill}")
    };

    rsx!(
        rect {
            direction: "vertical",
            rect {
                width: "{width}",
                onpointerenter,
                onpointerleave,
                onclick,
                onglobalkeydown,
                onkeydown,
                margin: "{margin}",
                a11y_id,
                a11y_member_of,
                background: "{background}",
                color: "{font_theme.color}",
                corner_radius: "8",
                padding: "6 16",
                border,
                direction: "horizontal",
                main_align: "center",
                cross_align: "center",
                {selected_item}
                ArrowIcon {
                    rotate: "0",
                    fill: "{arrow_fill}",
                    theme: theme_with!(IconTheme {
                        margin : "0 0 0 8".into(),
                    })
                }
            }
            if *opened.read() {
                rect {
                    height: "0",
                    width: "0",
                    rect {
                        width: "100v",
                        margin: "4 0 0 0",
                        rect {
                            onglobalpointerup,
                            onglobalkeydown,
                            layer: "overlay",
                            margin: "{margin}",
                            border: "1 inner {border_fill}",
                            overflow: "clip",
                            corner_radius: "8",
                            background: "{dropdown_background}",
                            shadow: "0 2 4 0 rgb(0, 0, 0, 0.15)",
                            padding: "6",
                            content: "fit",
                            {children}
                        }
                    }
                }
            }
        }
    )
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn dropdown() {
        fn dropdown_app() -> Element {
            let values = use_hook(|| {
                vec![
                    "Value A".to_string(),
                    "Value B".to_string(),
                    "Value C".to_string(),
                ]
            });
            let mut selected_dropdown = use_signal(|| 0);

            rsx!(
                Dropdown {
                    selected_item: rsx!( label { "{values[selected_dropdown()]}" } ),
                    for (i, ch) in values.iter().enumerate() {
                        DropdownItem {
                            onpress: move |_| selected_dropdown.set(i),
                            label { "{ch}" }
                        }
                    }
                }
            )
        }

        let mut utils = launch_test(dropdown_app);
        let root = utils.root();
        let label = root.get(0).get(0).get(0);
        utils.wait_for_update().await;

        // Currently closed
        let start_size = utils.sdom().get().layout().size();

        // Default value
        assert_eq!(label.get(0).text(), Some("Value A"));

        // Open the dropdown
        utils.click_cursor((15., 15.)).await;
        utils.wait_for_update().await;

        // Now that the dropwdown is opened, there are more nodes in the layout
        assert!(utils.sdom().get().layout().size() > start_size);

        // Close the dropdown by clicking outside of it
        utils.click_cursor((200., 200.)).await;

        // Now the layout size is like in the begining
        assert_eq!(utils.sdom().get().layout().size(), start_size);

        // Open the dropdown again
        utils.click_cursor((15., 15.)).await;

        // Click on the second option
        utils.click_cursor((45., 90.)).await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        // Now the layout size is like in the begining, again
        assert_eq!(utils.sdom().get().layout().size(), start_size);

        // The second optio was selected
        assert_eq!(label.get(0).text(), Some("Value B"));
    }

    #[tokio::test]
    pub async fn dropdown_keyboard_navigation() {
        fn dropdown_keyboard_navigation_app() -> Element {
            let values = use_hook(|| {
                vec![
                    "Value A".to_string(),
                    "Value B".to_string(),
                    "Value C".to_string(),
                ]
            });
            let mut selected_dropdown = use_signal(|| 0);

            rsx!(
                Dropdown {
                    selected_item: rsx!( label { "{values[selected_dropdown()]}" } ),
                    for (i, ch) in values.iter().enumerate() {
                        DropdownItem {
                            onpress: move |_| selected_dropdown.set(i),
                            label { "{ch}" }
                        }
                    }
                }
            )
        }

        let mut utils = launch_test(dropdown_keyboard_navigation_app);
        let root = utils.root();
        let label = root.get(0).get(0).get(0);
        utils.wait_for_update().await;

        // Currently closed
        let start_size = utils.sdom().get().layout().size();

        // Default value
        assert_eq!(label.get(0).text(), Some("Value A"));

        // Open the dropdown
        utils.push_event(TestEvent::Keyboard {
            name: KeyboardEventName::KeyDown,
            key: Key::Tab,
            code: Code::Tab,
            modifiers: Modifiers::default(),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.push_event(TestEvent::Keyboard {
            name: KeyboardEventName::KeyDown,
            key: Key::Enter,
            code: Code::Enter,
            modifiers: Modifiers::default(),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        // Now that the dropwdown is opened, there are more nodes in the layout
        assert!(utils.sdom().get().layout().size() > start_size);

        // Close the dropdown by pressinc Esc
        utils.push_event(TestEvent::Keyboard {
            name: KeyboardEventName::KeyDown,
            key: Key::Escape,
            code: Code::Escape,
            modifiers: Modifiers::default(),
        });
        utils.wait_for_update().await;

        // Now the layout size is like in the begining
        assert_eq!(utils.sdom().get().layout().size(), start_size);

        // Open the dropdown again
        utils.push_event(TestEvent::Keyboard {
            name: KeyboardEventName::KeyDown,
            key: Key::Enter,
            code: Code::Enter,
            modifiers: Modifiers::default(),
        });
        utils.wait_for_update().await;

        // Click on the second option
        utils.push_event(TestEvent::Keyboard {
            name: KeyboardEventName::KeyDown,
            key: Key::Tab,
            code: Code::Tab,
            modifiers: Modifiers::default(),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.push_event(TestEvent::Keyboard {
            name: KeyboardEventName::KeyDown,
            key: Key::Tab,
            code: Code::Tab,
            modifiers: Modifiers::default(),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.push_event(TestEvent::Keyboard {
            name: KeyboardEventName::KeyDown,
            key: Key::Enter,
            code: Code::Enter,
            modifiers: Modifiers::default(),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        // Close with Escape
        utils.push_event(TestEvent::Keyboard {
            name: KeyboardEventName::KeyDown,
            key: Key::Escape,
            code: Code::Escape,
            modifiers: Modifiers::default(),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        // Now the layout size is like in the begining, again
        assert_eq!(utils.sdom().get().layout().size(), start_size);

        // The second option was selected
        assert_eq!(label.get(0).text(), Some("Value B"));
    }
}
