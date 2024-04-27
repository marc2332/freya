use crate::TickIcon;
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{use_applied_theme, CheckboxTheme, CheckboxThemeWith};

/// Controlled `Checkbox` component.
///
/// # Styling
/// Inherits the [`CheckboxTheme`](freya_hooks::CheckboxTheme) theme.
///
/// # Example
///
/// ```no_run
/// # use std::collections::HashSet;
/// # use freya::prelude::*;
/// #[derive(PartialEq, Eq, Hash)]
/// enum Choice {
///     FirstChoice,
///     SecondChoice,
/// }
///
/// fn app() -> Element {
///     let mut selected = use_signal::<HashSet<Choice>>(HashSet::default);
///     rsx!(
///         Tile {
///             onselect: move |_| {
///                 if selected.read().contains(&Choice::FirstChoice) {
///                     selected.write().remove(&Choice::FirstChoice);
///                 } else {
///                     selected.write().insert(Choice::FirstChoice);
///                 }
///             },
///             leading: rsx!(
///                 Checkbox {
///                     selected: selected.read().contains(&Choice::FirstChoice),
///                 },
///             ),
///             label { "First choice" }
///         }
///         Tile {
///             onselect: move |_| {
///                 if selected.read().contains(&Choice::SecondChoice) {
///                     selected.write().remove(&Choice::SecondChoice);
///                 } else {
///                     selected.write().insert(Choice::SecondChoice);
///                 }
///             },
///             leading: rsx!(
///                 Checkbox {
///                     selected: selected.read().contains(&Choice::SecondChoice),
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
pub fn Checkbox(
    /// Indicate whether this checkbox is selected or not.
    selected: bool,
    /// Theme override.
    theme: Option<CheckboxThemeWith>,
) -> Element {
    let CheckboxTheme {
        unselected_fill,
        selected_fill,
        selected_icon_fill,
    } = use_applied_theme!(&theme, checkbox);
    let (fill, border) = if selected {
        (selected_fill.as_ref(), selected_fill.as_ref())
    } else {
        ("transparent", unselected_fill.as_ref())
    };

    rsx!(
        rect {
            width: "18",
            height: "18",
            padding: "4",
            main_align: "center",
            cross_align: "center",
            corner_radius: "4",
            border: "2 solid {border}",
            background: "{fill}",
            if selected {
                TickIcon {
                    fill: selected_icon_fill
                }
            }
        }
    )
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use dioxus::prelude::use_signal;
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn checkbox() {
        #[derive(PartialEq, Eq, Hash)]
        enum Choice {
            FirstChoice,
            SecondChoice,
            ThirdChoice,
        }

        fn checkbox_app() -> Element {
            let mut selected = use_signal::<HashSet<Choice>>(HashSet::default);

            rsx!(
                Tile {
                    onselect: move |_| {
                        if selected.read().contains(&Choice::FirstChoice) {
                            selected.write().remove(&Choice::FirstChoice);
                        } else {
                            selected.write().insert(Choice::FirstChoice);
                        }
                    },
                    leading: rsx!(
                        Checkbox {
                            selected: selected.read().contains(&Choice::FirstChoice),
                        },
                    ),
                    label { "First choice" }
                }
                Tile {
                    onselect: move |_| {
                        if selected.read().contains(&Choice::SecondChoice) {
                            selected.write().remove(&Choice::SecondChoice);
                        } else {
                            selected.write().insert(Choice::SecondChoice);
                        }
                    },
                    leading: rsx!(
                        Checkbox {
                            selected: selected.read().contains(&Choice::SecondChoice),
                        },
                    ),
                    label { "Second choice" }
                }
                Tile {
                    onselect: move |_| {
                        if selected.read().contains(&Choice::ThirdChoice) {
                            selected.write().remove(&Choice::ThirdChoice);
                        } else {
                            selected.write().insert(Choice::ThirdChoice);
                        }
                    },
                    leading: rsx!(
                        Checkbox {
                            selected: selected.read().contains(&Choice::ThirdChoice),
                        },
                    ),
                    label { "Third choice" }
                }
            )
        }

        let mut utils = launch_test(checkbox_app);
        let root = utils.root();
        utils.wait_for_update().await;

        // If the inner square exists it means that the Checkbox is selected, otherwise it isn't
        assert!(root.get(0).get(0).get(0).get(0).is_placeholder());
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
        assert!(root.get(1).get(0).get(0).get(0).is_element());
        assert!(root.get(2).get(0).get(0).get(0).is_element());

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (10.0, 10.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (10.0, 50.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;

        assert!(root.get(0).get(0).get(0).get(0).is_element());
        assert!(root.get(1).get(0).get(0).get(0).is_placeholder());
        assert!(root.get(2).get(0).get(0).get(0).is_element());
    }
}
