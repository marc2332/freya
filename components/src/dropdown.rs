use std::fmt::Display;

use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use freya_elements::MouseEvent;

/// `DropdownItem` component.
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

    let background = if &*selected.read() == value {
        "rgb(200, 200, 200)"
    } else {
        "white"
    };

    render!(rect {
        width: "100%",
        height: "35",
        color: "black",
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
#[allow(non_snake_case)]
pub fn Dropdown<'a, T>(cx: Scope<'a, DropdownProps<'a, T>>) -> Element<'a>
where
    T: PartialEq + Clone + Display + 'static,
{
    use_shared_state_provider(cx, || cx.props.value.clone());
    let selected = use_shared_state::<T>(cx).unwrap();

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
                    background: "white",
                    shadow: "0 0 100 6 black",
                    padding: "15",
                    &cx.props.children
                }
            }
        )
    } else {
        render!(
            container {
                radius: "3",
                onclick: move |_| opened.set(true),
                width: "70",
                height: "50",
                padding: "5",
                label {
                    align: "center",
                    color: "black",
                    "{selected.read()}"
                }
                rect {
                    width: "100%",
                    height: "2",
                    background: "black",
                }
            }
        )
    }
}
