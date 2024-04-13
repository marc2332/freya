use std::fmt::Display;

use crate::icons::ArrowIcon;
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::keyboard::Key;
use freya_elements::events::{KeyboardEvent, MouseEvent};

use freya_hooks::{
    theme_with, use_applied_theme, use_focus, use_platform, DropdownItemThemeWith, DropdownTheme,
    DropdownThemeWith, IconThemeWith,
};
use winit::window::CursorIcon;

/// Properties for the [`DropdownItem`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DropdownItemProps<T: 'static + Clone + PartialEq> {
    /// Theme override.
    pub theme: Option<DropdownItemThemeWith>,
    /// Selectable items, like [`DropdownItem`]
    pub children: Element,
    /// Selected value.
    pub value: T,
    /// Handler for the `onclick` event.
    pub onclick: Option<EventHandler<()>>,
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
pub fn DropdownItem<T: Clone>(
    DropdownItemProps {
        theme,
        children,
        value,
        onclick,
    }: DropdownItemProps<T>,
) -> Element
where
    T: PartialEq + 'static,
{
    let selected = use_context::<Signal<T>>();
    let theme = use_applied_theme!(&theme, dropdown_item);
    let focus = use_focus();
    let mut status = use_signal(DropdownItemStatus::default);
    let platform = use_platform();

    let focus_id = focus.attribute();
    let is_focused = focus.is_focused();
    let is_selected = *selected.read() == value;

    let background = match *status.read() {
        _ if is_selected => theme.select_background,
        _ if is_focused => theme.hover_background,
        DropdownItemStatus::Hovering => theme.hover_background,
        DropdownItemStatus::Idle => theme.background,
    };
    let color = theme.font_theme.color;

    use_drop(move || {
        if *status.peek() == DropdownItemStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Pointer);
        status.set(DropdownItemStatus::Hovering);
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(DropdownItemStatus::default());
    };

    let onkeydown = {
        to_owned![onclick];
        move |ev: KeyboardEvent| {
            if ev.key == Key::Enter && is_focused {
                if let Some(onclick) = &onclick {
                    onclick.call(())
                }
            }
        }
    };

    let onclick = move |_: MouseEvent| {
        if let Some(onclick) = &onclick {
            onclick.call(())
        }
    };

    rsx!(
        rect {
            width: "fill-min",
            color: "{color}",
            focus_id,
            role: "button",
            background: "{background}",
            padding: "6 22 6 16",
            corner_radius: "6",
            main_align: "center",
            onmouseenter,
            onmouseleave,
            onclick,
            onkeydown,
            {children}
        }
    )
}

/// Properties for the [`Dropdown`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps<T: 'static + Clone + PartialEq> {
    /// Theme override.
    pub theme: Option<DropdownThemeWith>,
    /// Selectable items, like [`DropdownItem`]
    pub children: Element,
    /// Selected value.
    pub value: T,
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

/// Select from multiple options, use alongside [`DropdownItem`].
///
/// # Styling
/// Inherits the [`DropdownTheme`](freya_hooks::DropdownTheme) theme.
///
/// # Example
/// ```no_run
/// # use freya::prelude::*;
///
/// fn app() -> Element {
///     let values = use_hook(|| vec!["A".to_string(), "B".to_string(), "C".to_string()]);
///     let mut selected_dropdown = use_signal(|| "A".to_string());
///     rsx!(
///         Dropdown {
///             value: selected_dropdown.read().clone(),
///             for ch in values {
///                 DropdownItem {
///                     value: ch.to_string(),
///                     onclick: {
///                         to_owned![ch];
///                         move |_| selected_dropdown.set(ch.clone())
///                     },
///                     label { "{ch}" }
///                 }
///             }
///         }
///     )
/// }
/// ```
#[allow(non_snake_case)]
pub fn Dropdown<T>(props: DropdownProps<T>) -> Element
where
    T: PartialEq + Clone + Display + 'static,
{
    let mut selected = use_context_provider(|| Signal::new(props.value.clone()));
    let theme = use_applied_theme!(&props.theme, dropdown);
    let mut focus = use_focus();
    let mut status = use_signal(DropdownStatus::default);
    let mut opened = use_signal(|| false);
    let platform = use_platform();

    let is_opened = *opened.read();
    let is_focused = focus.is_focused();
    let focus_id = focus.attribute();

    // Update the provided value if the passed value changes
    use_effect(use_reactive(&props.value, move |value| {
        *selected.write() = value;
    }));

    use_drop(move || {
        if *status.peek() == DropdownStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    // Close the dropdown if clicked anywhere
    let onglobalclick = move |_: MouseEvent| {
        opened.set(false);
    };

    let onclick = move |_| {
        focus.focus();
        opened.set(true)
    };

    let onkeydown = move |e: KeyboardEvent| {
        match e.key {
            // Close when `Escape` key is pressed
            Key::Escape => {
                opened.set(false);
            }
            // Open the dropdown items when the `Enter` key is pressed
            Key::Enter if is_focused && !is_opened => {
                opened.set(true);
            }
            _ => {}
        }
    };

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Pointer);
        status.set(DropdownStatus::Hovering);
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(DropdownStatus::default());
    };

    let DropdownTheme {
        font_theme,
        dropdown_background,
        background_button,
        hover_background,
        border_fill,
        arrow_fill,
    } = &theme;

    let button_background = match *status.read() {
        DropdownStatus::Hovering => hover_background,
        DropdownStatus::Idle => background_button,
    };

    let selected = selected.read().to_string();

    rsx!(
        rect {
            onmouseenter,
            onmouseleave,
            onclick,
            onkeydown,
            margin: "4",
            focus_id,
            background: "{button_background}",
            color: "{font_theme.color}",
            corner_radius: "8",
            padding: "8 16",
            border: "1 solid {border_fill}",
            shadow: "0 4 5 0 rgb(0, 0, 0, 0.1)",
            direction: "horizontal",
            main_align: "center",
            cross_align: "center",
            label {
                "{selected}"
            }
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
                rect {
                    onglobalclick,
                    onkeydown,
                    layer: "-99",
                    margin: "4",
                    border: "1 solid {border_fill}",
                    overflow: "clip",
                    corner_radius: "8",
                    background: "{dropdown_background}",
                    shadow: "0 4 5 0 rgb(0, 0, 0, 0.3)",
                    padding: "6",
                    content: "fit",
                    {props.children}
                }
            }
        }
    )
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_testing::prelude::*;
    use winit::event::MouseButton;

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
            let mut selected_dropdown = use_signal(|| "Value A".to_string());

            rsx!(
                Dropdown {
                    value: selected_dropdown.read().clone(),
                    for ch in values {
                        DropdownItem {
                            value: ch.clone(),
                            onclick: {
                                to_owned![ch];
                                move |_| selected_dropdown.set(ch.clone())
                            },
                            label { "{ch}" }
                        }
                    }
                }
            )
        }

        let mut utils = launch_test(dropdown_app);
        let root = utils.root();
        let label = root.get(0).get(0);
        utils.wait_for_update().await;

        // Currently closed
        let start_size = utils.sdom().get().layout().size();

        // Default value
        assert_eq!(label.get(0).text(), Some("Value A"));

        // Open the dropdown
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (5.0, 5.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        // Now that the dropwdown is opened, there are more nodes in the layout
        assert!(utils.sdom().get().layout().size() > start_size);

        // Close the dropdown by clicking outside of it
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (200.0, 200.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        // Now the layout size is like in the begining
        assert_eq!(utils.sdom().get().layout().size(), start_size);

        // Open the dropdown again
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (5.0, 5.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        // Click on the second option
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (45.0, 100.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        // Now the layout size is like in the begining, again
        assert_eq!(utils.sdom().get().layout().size(), start_size);

        // The second optio was selected
        assert_eq!(label.get(0).text(), Some("Value B"));
    }
}
