use std::fmt::Display;

use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use freya_elements::MouseEvent;
use freya_hooks::use_get_theme;

/// `DropdownItem` component.
///
/// # Styling
/// Inherits the [`DropdownItemTheme`](freya_hooks::DropdownTheme) theme.
#[allow(non_snake_case)]
#[inline_props]
pub fn DropdownItem<'a, T>(
    cx: Scope<'a>,
    children: Element<'a>,
    value: T,
    onclick: Option<EventHandler<'a, ()>>,
) -> Element<'a>
where
    T: PartialEq + 'static,
{
    let selected = use_shared_state::<T>(cx).unwrap();
    let theme = use_get_theme(cx);
    let dropdownitem_theme = &theme.dropdown_item;

    let background = if &*selected.read() == value {
        dropdownitem_theme.hover_background
    } else {
        dropdownitem_theme.background
    };

    render!(rect {
        color: "{dropdownitem_theme.font_theme.color}",
        width: "100%",
        height: "35",
        background: background,
        padding: "12",
        radius: "3",
        onclick: move |_| {
            if let Some(onclick) = onclick {
                onclick.call(());
            }
        },
        children
    })
}

/// [`Dropdown`] component properties.
#[derive(Props)]
pub struct DropdownProps<'a, T: 'static> {
    children: Element<'a>,
    value: T,
}

/// `Dropdown` component.
///
/// # Props
/// See [`DropdownProps`].
///
/// # Styling
/// Inherits the [`DropdownTheme`](freya_hooks::DropdownTheme) theme.
#[allow(non_snake_case)]
pub fn Dropdown<'a, T>(cx: Scope<'a, DropdownProps<'a, T>>) -> Element<'a>
where
    T: PartialEq + Clone + Display + 'static,
{
    use_shared_state_provider(cx, || cx.props.value.clone());
    let selected = use_shared_state::<T>(cx).unwrap();
    let theme = use_get_theme(cx);
    let dropdown_theme = &theme.dropdown;
    let background_button = use_state(cx, || <&str>::clone(&dropdown_theme.background_button));
    let set_background_button = background_button.setter();

    use_effect(
        cx,
        &dropdown_theme.clone(),
        move |dropdown_theme| async move {
            set_background_button(dropdown_theme.background_button);
        },
    );

    // Update the provided value if the passed value changes
    use_effect(cx, &cx.props.value, move |value| {
        *selected.write() = value;
        async move {}
    });

    let opened = use_state(cx, || false);

    // Close the dropdown if clicked anywhere
    let onglobalclick = move |_: MouseEvent| {
        opened.set(false);
    };

    if *opened.get() {
        render!(
            rect {
                width: "70",
                height: "50",
                container {
                    layer: "-1",
                    radius: "3",
                    onglobalclick: onglobalclick,
                    width: "130",
                    height: "auto",
                    background: *background_button.get(),
                    shadow: "0 0 100 6 black",
                    padding: "15",
                    &cx.props.children
                }
            }
        )
    } else {
        render!(
            container {
                color: "{dropdown_theme.font_theme.color}",
                radius: "3",
                onclick: move |_| opened.set(true),
                width: "70",
                height: "50",
                padding: "5",
                label {
                    align: "center",
                    "{selected.read()}"
                }
                rect {
                    width: "100%",
                    height: "2",
                    background: "{dropdown_theme.font_theme.color}",
                }
            }
        )
    }
}
