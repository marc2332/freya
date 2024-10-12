use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{
    use_animation,
    use_applied_theme,
    AnimNum,
    Ease,
    Function,
    SnackBarTheme,
    SnackBarThemeWith,
};

/// `SnackBar` component. Use in combination with other components.
///
/// # Styling
/// Inherits the [`SnackBarTheme`](freya_hooks::SnackBarTheme) theme.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let mut open = use_signal(|| false);
///
///     rsx!(
///         rect {
///             height: "100%",
///             width: "100%",
///             Button {
///                 onpress: move |_| open.toggle(),
///                 label { "Open" }
///             }
///             SnackBar {
///                 open,
///                 label {
///                     "Hello, World!"
///                 }
///             }
///         }
///     )
/// }
/// # use freya_testing::prelude::*;
/// # let mut utils = launch_doc_with_utils(|| {
/// #   rsx!(
/// #       Preview {
/// #           SnackBar {
/// #               open: true,
/// #               label {
/// #                   "Hello, World!"
/// #               }
/// #           }
/// #       }
/// #   )
/// # }, (200., 150.).into());
/// # utils.blocking_wait_for_update();
/// # utils.blocking_wait_for_update();
/// # utils.blocking_wait_for_update();
/// # utils.blocking_wait_for_update();
/// # utils.blocking_wait_for_update();
/// # utils.save_snapshot("./images/gallery_snackbar.png");
/// ```
///
/// # Preview
/// ![Snackbar Preview][snackbar]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("snackbar", "images/gallery_snackbar.png")
)]
#[allow(non_snake_case)]
#[component]
pub fn SnackBar(
    /// Inner children of the SnackBar.
    children: Element,
    /// Open or not the SnackBar. You can pass a [ReadOnlySignal] as well.
    open: ReadOnlySignal<bool>,
    /// Theme override.
    theme: Option<SnackBarThemeWith>,
) -> Element {
    let animation = use_animation(|ctx| {
        ctx.with(
            AnimNum::new(50., 0.)
                .time(200)
                .ease(Ease::Out)
                .function(Function::Expo),
        )
    });

    use_effect(move || {
        if open() {
            animation.start();
        } else if animation.peek_has_run_yet() {
            animation.reverse();
        }
    });

    let offset_y = animation.get().read().as_f32();

    rsx!(
        rect {
            width: "100%",
            height: "40",
            position: "absolute",
            position_bottom: "0",
            offset_y: "{offset_y}",
            overflow: "clip",
            SnackBarBox {
                theme,
                {children}
            }
        }
    )
}

#[doc(hidden)]
#[allow(non_snake_case)]
#[component]
pub fn SnackBarBox(children: Element, theme: Option<SnackBarThemeWith>) -> Element {
    let SnackBarTheme { background, color } = use_applied_theme!(&theme, snackbar);

    rsx!(
        rect {
            width: "fill",
            height: "40",
            background: "{background}",
            overflow: "clip",
            padding: "10",
            color: "{color}",
            direction: "horizontal",
            layer: "-1000",
            {children}
        }
    )
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use dioxus::prelude::use_signal;
    use freya::prelude::*;
    use freya_testing::prelude::*;
    use tokio::time::sleep;

    #[tokio::test]
    pub async fn snackbar() {
        fn snackbar_app() -> Element {
            let mut open = use_signal(|| false);

            rsx!(
                rect {
                    height: "100%",
                    width: "100%",
                    Button {
                        onpress: move |_|  open.toggle(),
                        label { "Open" }
                    }
                    SnackBar {
                        open,
                        label {
                            "Hello, World!"
                        }
                    }
                }
            )
        }

        let mut utils = launch_test(snackbar_app);
        let root = utils.root();
        let snackbar_box = root.get(0).get(1).get(0);
        utils.wait_for_update().await;

        // Snackbar is closed.
        assert!(!snackbar_box.is_visible());

        // Open the snackbar by clicking at the button
        utils.click_cursor((15., 15.)).await;

        // Wait a bit for the snackbar to open up
        utils.wait_for_update().await;
        sleep(Duration::from_millis(15)).await;
        utils.wait_for_update().await;

        // Snackbar is visible.
        assert!(snackbar_box.is_visible());
    }
}
