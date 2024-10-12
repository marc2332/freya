use dioxus::prelude::*;
use freya_elements::{
    elements as dioxus_elements,
    events::KeyboardEvent,
};
use freya_hooks::{
    use_applied_theme,
    use_focus,
    RadioTheme,
    RadioThemeWith,
};

/// Controlled `Radio` component.
///
/// # Styling
/// Inherits the [`RadioTheme`](freya_hooks::RadioTheme) theme.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// #[derive(PartialEq)]
/// enum Choice {
///     First,
///     Second,
/// }
///
/// fn app() -> Element {
///     let mut selected = use_signal(|| Choice::First);
///     rsx!(
///         Tile {
///             onselect: move |_| selected.set(Choice::First),
///             leading: rsx!(
///                 Radio {
///                     selected: *selected.read() == Choice::First,
///                 },
///             ),
///             label { "First choice" }
///         }
///         Tile {
///             onselect: move |_| selected.set(Choice::Second),
///             leading: rsx!(
///                 Radio {
///                     selected: *selected.read() == Choice::Second,
///                 },
///             ),
///             label { "Second choice" }
///         }
///     )
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rsx!(
/// #       Preview {
/// #           Radio {
/// #               selected: true
/// #           }
/// #       }
/// #   )
/// # }, (200., 150.).into(), "./images/gallery_radio.png");
/// ```
/// # Preview
/// ![Radio Preview][radio]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_doc_image!("radio", "images/gallery_radio.png")
)]
#[allow(non_snake_case)]
#[component]
pub fn Radio(
    /// Indicate whether this radio is selected or not.
    selected: bool,
    /// Theme override.
    theme: Option<RadioThemeWith>,
) -> Element {
    let focus = use_focus();
    let RadioTheme {
        unselected_fill,
        selected_fill,
        border_fill,
    } = use_applied_theme!(&theme, radio);
    let fill = if selected {
        selected_fill
    } else {
        unselected_fill
    };
    let border = if focus.is_selected() {
        format!("2 inner {fill}, 4 outer {border_fill}")
    } else {
        format!("2 inner {fill}")
    };

    let onkeydown = move |_: KeyboardEvent| {};

    rsx!(
        rect {
            a11y_id: focus.attribute(),
            width: "18",
            height: "18",
            border,
            padding: "4",
            main_align: "center",
            cross_align: "center",
            corner_radius: "99",
            onkeydown,
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
            First,
            Second,
            Third,
        }

        fn radio_app() -> Element {
            let mut selected = use_signal(|| Choice::First);

            rsx!(
                Tile {
                    onselect: move |_| selected.set(Choice::First),
                    leading: rsx!(
                        Radio {
                            selected: *selected.read() == Choice::First,
                        },
                    ),
                    label { "First choice" }
                }
                Tile {
                    onselect: move |_| selected.set(Choice::Second),
                    leading: rsx!(
                        Radio {
                            selected: *selected.read() == Choice::Second,
                        },
                    ),
                    label { "Second choice" }
                }
                Tile {
                    onselect: move |_| selected.set(Choice::Third),
                    leading: rsx!(
                        Radio {
                            selected: *selected.read() == Choice::Third,
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

        utils.click_cursor((20., 50.)).await;

        assert!(root.get(0).get(0).get(0).get(0).is_placeholder());
        assert!(root.get(1).get(0).get(0).get(0).is_element());
        assert!(root.get(2).get(0).get(0).get(0).is_placeholder());

        utils.click_cursor((10., 90.)).await;

        assert!(root.get(0).get(0).get(0).get(0).is_placeholder());
        assert!(root.get(1).get(0).get(0).get(0).is_placeholder());
        assert!(root.get(2).get(0).get(0).get(0).is_element());

        utils.click_cursor((10., 10.)).await;

        assert!(root.get(0).get(0).get(0).get(0).is_element());
        assert!(root.get(1).get(0).get(0).get(0).is_placeholder());
        assert!(root.get(2).get(0).get(0).get(0).is_placeholder());
    }
}
