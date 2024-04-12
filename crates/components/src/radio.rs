use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{use_applied_theme, RadioTheme, RadioThemeWith};

/// Controlled `Radio` component.
///
/// # Styling
/// Inherits the [`RadioTheme`](freya_hooks::RadioTheme) theme.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// #[derive(PartialEq)]
/// enum Choice {
///     FirstChoice,
///     SecondChoice,
/// }
///
/// fn app() -> Element {
///     let mut selected = use_signal(|| Choice::FirstChoice);
///     rsx!(
///         Tile {
///             onselect: move |_| selected.set(Choice::FirstChoice),
///             leading: rsx!(
///                 Radio {
///                     selected: *selected.read() == Choice::FirstChoice,
///                 },
///             ),
///             label { "First choice" }
///         }
///         Tile {
///             onselect: move |_| selected.set(Choice::SecondChoice),
///             leading: rsx!(
///                 Radio {
///                     selected: *selected.read() == Choice::SecondChoice,
///                 },
///             ),
///             label { "Second choice" }
///         }
///     )
/// }
/// ```
///
#[allow(non_snake_case)]
#[component]
pub fn Radio(
    /// Indicate whether this radio is selected or not.
    selected: bool,
    /// Theme override.
    theme: Option<RadioThemeWith>,
) -> Element {
    let RadioTheme {
        unselected_fill,
        selected_fill,
    } = use_applied_theme!(&theme, radio);
    let fill = if selected {
        selected_fill
    } else {
        unselected_fill
    };

    rsx!(
        rect {
            width: "18",
            height: "18",
            border: "2 solid {fill}",
            padding: "4",
            main_align: "center",
            cross_align: "center",
            corner_radius: "99",
            if selected {
                rect {
                    width: "10",
                    height: "10",
                    background: "{fill}",
                    corner_radius: "99",
                }
            }
        }
    )
}

#[cfg(test)]
mod test {
    use dioxus::prelude::use_signal;
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn radio() {
        #[derive(PartialEq)]
        enum Choice {
            FirstChoice,
            SecondChoice,
            ThirdChoice,
        }

        fn radio_app() -> Element {
            let mut selected = use_signal(|| Choice::FirstChoice);

            rsx!(
                Tile {
                    onselect: move |_| selected.set(Choice::FirstChoice),
                    leading: rsx!(
                        Radio {
                            selected: *selected.read() == Choice::FirstChoice,
                        },
                    ),
                    label { "First choice" }
                }
                Tile {
                    onselect: move |_| selected.set(Choice::SecondChoice),
                    leading: rsx!(
                        Radio {
                            selected: *selected.read() == Choice::SecondChoice,
                        },
                    ),
                    label { "Second choice" }
                }
                Tile {
                    onselect: move |_| selected.set(Choice::ThirdChoice),
                    leading: rsx!(
                        Radio {
                            selected: *selected.read() == Choice::ThirdChoice,
                        },
                    ),
                    label { "Third choice" }
                }
            )
        }

        let mut utils = launch_test(radio_app);
        let root = utils.root();
        utils.wait_for_update().await;

        // If the inner circle exists it means that the Radio is activated, otherwise it isn't
        assert!(root.get(0).get(0).get(0).get(0).is_element());
        assert!(root.get(1).get(0).get(0).get(0).is_placeholder());
        assert!(root.get(2).get(0).get(0).get(0).is_placeholder());

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (20.0, 50.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        assert!(root.get(0).get(0).get(0).get(0).is_placeholder());
        assert!(root.get(1).get(0).get(0).get(0).is_element());
        assert!(root.get(2).get(0).get(0).get(0).is_placeholder());

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (10.0, 90.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        assert!(root.get(0).get(0).get(0).get(0).is_placeholder());
        assert!(root.get(1).get(0).get(0).get(0).is_placeholder());
        assert!(root.get(2).get(0).get(0).get(0).is_element());

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (10.0, 10.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        assert!(root.get(0).get(0).get(0).get(0).is_element());
        assert!(root.get(1).get(0).get(0).get(0).is_placeholder());
        assert!(root.get(2).get(0).get(0).get(0).is_placeholder());
    }
}
