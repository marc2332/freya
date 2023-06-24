use std::fmt::Display;

use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::keyboard::Key;
use freya_elements::events::{KeyboardEvent, MouseEvent};
use freya_hooks::{use_focus, use_get_theme};

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
pub enum DropdownItemState {
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
    let state = use_state(cx, DropdownItemState::default);

    let focus_id = focus.attribute(cx);
    let is_focused = focus.is_focused();
    let is_selected = *selected.read() == cx.props.value;

    let background = match *state.get() {
        _ if is_selected => theme.dropdown_item.select_background,
        _ if is_focused => theme.dropdown_item.hover_background,
        DropdownItemState::Hovering => theme.dropdown_item.hover_background,
        DropdownItemState::Idle => theme.dropdown_item.background,
    };
    let color = theme.dropdown_item.font_theme.color;

    let onclick = move |_: MouseEvent| {
        if let Some(onclick) = &cx.props.onclick {
            onclick.call(())
        }
    };

    let onmouseenter = move |_| {
        state.set(DropdownItemState::Hovering);
    };

    let onmouseleave = move |_| {
        state.set(DropdownItemState::default());
    };

    let onkeydown = move |ev: KeyboardEvent| {
        if ev.key == Key::Enter && is_focused {
            if let Some(onclick) = &cx.props.onclick {
                onclick.call(())
            }
        }
    };

    render!(rect {
        color: color,
        width: "100%",
        height: "35",
        focus_id: focus_id,
        role: "button",
        background: background,
        padding: "6",
        radius: "3",
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
pub enum DropdownState {
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
    let state = use_state(cx, DropdownState::default);
    let opened = use_state(cx, || false);

    let is_opened = *opened.get();
    let is_focused = focus.is_focused();
    let focus_id = focus.attribute(cx);

    let background = match *state.get() {
        DropdownState::Hovering => theme.dropdown.hover_background,
        DropdownState::Idle => theme.dropdown.background_button,
    };
    let color = theme.dropdown.font_theme.color;

    // Update the provided value if the passed value changes
    use_effect(cx, &cx.props.value, move |value| {
        *selected.write() = value;
        async move {}
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

    if *opened.get() {
        render!(
            rect {
                width: "70",
                height: "50",
                rect {
                    overflow: "clip",
                    focus_id: focus_id,
                    layer: "-1",
                    radius: "3",
                    onglobalclick: onglobalclick,
                    onkeydown: onkeydown,
                    width: "130",
                    height: "auto",
                    background: background,
                    shadow: "0 0 20 0 rgb(0, 0, 0, 100)",
                    padding: "7",
                    &cx.props.children
                }
            }
        )
    } else {
        render!(
            rect {
                overflow: "clip",
                focus_id: focus_id,
                background: background,
                color: color,
                radius: "3",
                onclick: onclick,
                onkeydown: onkeydown,
                width: "70",
                height: "auto",
                padding: "7",
                label {
                    align: "center",
                    "{selected.read()}"
                }
                rect {
                    width: "100%",
                    height: "2",
                    background: color,
                }
            }
        )
    }
}
