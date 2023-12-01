use std::fmt::Display;

use crate::icons::ArrowIcon;
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::keyboard::Key;
use freya_elements::events::{KeyboardEvent, MouseEvent};
use freya_hooks::{use_focus, use_get_theme, use_platform};
use winit::window::CursorIcon;

/// [`DropdownItem`] component properties.
#[derive(Props)]
pub struct DropdownItemProps<'a, T: 'static> {
    /// Selectable items, like [`DropdownItem`]
    children: Element<'a>,
    /// Selected value.
    value: T,
    /// Handler for the `onclick` event.
    #[props(optional)]
    onclick: Option<EventHandler<'a, ()>>,
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

/// `DropdownItem` component.
///
/// # Props
/// See [`DropdownItemProps`].
///
/// # Styling
/// Inherits the [`DropdownItemTheme`](freya_hooks::DropdownItemTheme) theme.
#[allow(non_snake_case)]
pub fn DropdownItem<'a, T>(cx: Scope<'a, DropdownItemProps<'a, T>>) -> Element<'a>
where
    T: PartialEq + 'static,
{
    let selected = use_shared_state::<T>(cx).unwrap();
    let theme = use_get_theme(cx);
    let focus = use_focus(cx);
    let status = use_state(cx, DropdownItemStatus::default);
    let platform = use_platform(cx);

    let focus_id = focus.attribute(cx);
    let is_focused = focus.is_focused();
    let is_selected = *selected.read() == cx.props.value;

    let background = match *status.get() {
        _ if is_selected => theme.dropdown_item.select_background,
        _ if is_focused => theme.dropdown_item.hover_background,
        DropdownItemStatus::Hovering => theme.dropdown_item.hover_background,
        DropdownItemStatus::Idle => theme.dropdown_item.background,
    };
    let color = theme.dropdown_item.font_theme.color;

    use_on_unmount(cx, {
        to_owned![status, platform];
        move || {
            if *status.current() == DropdownItemStatus::Hovering {
                platform.set_cursor(CursorIcon::default());
            }
        }
    });

    let onclick = move |_: MouseEvent| {
        if let Some(onclick) = &cx.props.onclick {
            onclick.call(())
        }
    };

    let onmouseenter = {
        to_owned![platform];
        move |_| {
            platform.set_cursor(CursorIcon::Hand);
            status.set(DropdownItemStatus::Hovering);
        }
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(DropdownItemStatus::default());
    };

    let onkeydown = move |ev: KeyboardEvent| {
        if ev.key == Key::Enter && is_focused {
            if let Some(onclick) = &cx.props.onclick {
                onclick.call(())
            }
        }
    };

    render!(rect {
        color: "{color}",
        focus_id: focus_id,
        role: "button",
        background: "{background}",
        padding: "6 22 6 16",
        corner_radius: "6",
        main_align: "center",
        cross_align: "center",
        onmouseenter: onmouseenter,
        onmouseleave: onmouseleave,
        onclick: onclick,
        onkeydown: onkeydown,
        &cx.props.children
    })
}

/// [`Dropdown`] component properties.
#[derive(Props)]
pub struct DropdownProps<'a, T: 'static> {
    /// Selectable items, like [`DropdownItem`]
    children: Element<'a>,
    /// Selected value.
    value: T,
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

/// `Dropdown` component.
///
/// # Props
/// See [`DropdownProps`].
///
/// # Styling
/// Inherits the [`DropdownTheme`](freya_hooks::DropdownTheme) theme.
///
/// # Example
/// ```no_run
/// # use freya::prelude::*;
///
/// fn app(cx: Scope) -> Element {
///     let values = cx.use_hook(|| vec!["A".to_string(), "B".to_string(), "C".to_string()]);
///     let selected_dropdown = use_state(cx, || "A".to_string());
///     render!(
///         Dropdown {
///             value: selected_dropdown.get().clone(),
///             values.iter().map(|ch| {
///                 rsx!(
///                     DropdownItem {
///                         value: ch.to_string(),
///                         onclick: move |_| selected_dropdown.set(ch.to_string()),
///                         label { "{ch}" }
///                     }
///                 )
///             })
///         }
///     )
/// }
/// ```
#[allow(non_snake_case)]
pub fn Dropdown<'a, T>(cx: Scope<'a, DropdownProps<'a, T>>) -> Element<'a>
where
    T: PartialEq + Clone + Display + 'static,
{
    use_shared_state_provider(cx, || cx.props.value.clone());
    let selected = use_shared_state::<T>(cx).unwrap();
    let theme = use_get_theme(cx);
    let focus = use_focus(cx);
    let status = use_state(cx, DropdownStatus::default);
    let opened = use_state(cx, || false);
    let platform = use_platform(cx);

    let is_opened = *opened.get();
    let is_focused = focus.is_focused();
    let focus_id = focus.attribute(cx);

    // Update the provided value if the passed value changes
    use_memo(cx, &cx.props.value, move |value| {
        *selected.write() = value;
    });

    use_on_unmount(cx, {
        to_owned![status, platform];
        move || {
            if *status.current() == DropdownStatus::Hovering {
                platform.set_cursor(CursorIcon::default());
            }
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

    let onmouseenter = {
        to_owned![status, platform];
        move |_| {
            platform.set_cursor(CursorIcon::Hand);
            status.set(DropdownStatus::Hovering);
        }
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(DropdownStatus::default());
    };

    let desplegable_background = theme.dropdown.desplegable_background;
    let button_background = match *status.get() {
        DropdownStatus::Hovering => theme.dropdown.hover_background,
        DropdownStatus::Idle => theme.dropdown.background_button,
    };
    let border_fill = theme.dropdown.border_fill;
    let color = theme.dropdown.font_theme.color;
    let arrow_fill = theme.dropdown.arrow_fill;

    let selected = selected.read().to_string();

    render!(
        rect {
            onmouseenter: onmouseenter,
            onmouseleave: onmouseleave,
            onclick: onclick,
            onkeydown: onkeydown,
            margin: "4",
            focus_id: focus_id,
            background: "{button_background}",
            color: "{color}",
            corner_radius: "8",
            padding: "8 16",
            border: "1 solid {border_fill}",
            shadow: "0 4 5 0 rgb(0, 0, 0, 0.1)",
            direction: "horizontal",
            main_align: "center",
            cross_align: "center",
            label {
                text_align: "center",
                "{selected}"
            }
            ArrowIcon {
                rotate: "0",
                fill: "{arrow_fill}",
                margin: "0 0 0 8",
            }
        }
        if *opened.get() {
            rsx!(
                rect {
                    height: "0",
                    rect {
                        onglobalclick: onglobalclick,
                        onkeydown: onkeydown,
                        layer: "-99",
                        margin: "4",
                        border: "1 solid {border_fill}",
                        overflow: "clip",
                        corner_radius: "8",
                        background: "{desplegable_background}",
                        shadow: "0 4 5 0 rgb(0, 0, 0, 0.3)",
                        padding: "6",
                        &cx.props.children
                    }
                }
            )
        }
    )
}
