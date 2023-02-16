use std::fmt::Display;

use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use freya_elements::MouseEvent;
use freya_hooks::use_get_theme;

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
    let dropdownitem_theme = &theme.dropdown_item;

    let background = if *selected.read() == cx.props.value {
        dropdownitem_theme.hover_background
    } else {
        dropdownitem_theme.background
    };

    render!(rect {
        color: "{dropdownitem_theme.font_theme.color}",
        width: "100%",
        height: "35",
        background: background,
        padding: "6",
        radius: "3",
        onclick: move |_| {
            if let Some(onclick) = &cx.props.onclick {
                onclick.call(());
            }
        },
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
/// # use std::fmt::Display;
/// #[derive(PartialEq, Clone)]
/// enum Values {
///     A,
///     B,
///     C,
/// }
///
/// impl Display for Values {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         match self {
///             Values::A => f.write_str("Value A"),
///             Values::C => f.write_str("Value C"),
///             Values::B => f.write_str("Value B"),
///         }
///     }
/// }
///
/// fn app(cx: Scope) -> Element {
///     let selected_dropdown = use_state(cx, || Values::A);
///     render!(
///         Dropdown {
///             value: selected_dropdown.get().clone(),
///             DropdownItem { onclick: move |_| selected_dropdown.set(Values::A), value: Values::A, label { "A" } }
///             DropdownItem { onclick: move |_| selected_dropdown.set(Values::B), value: Values::B, label { "B" } }
///             DropdownItem { onclick: move |_| selected_dropdown.set(Values::C), value: Values::C, label { "C" } }
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
                    padding: "7",
                    &cx.props.children
                }
            }
        )
    } else {
        render!(
            container {
                background: dropdown_theme.desplegable_background,
                color: "{dropdown_theme.font_theme.color}",
                radius: "3",
                onclick: move |_| opened.set(true),
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
                    background: "{dropdown_theme.font_theme.color}",
                }
            }
        )
    }
}
