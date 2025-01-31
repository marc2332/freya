use dioxus::prelude::*;
use freya_elements::{
    self as dioxus_elements,
    events::MouseEvent,
};
use freya_hooks::{
    use_applied_theme,
    use_node_signal,
    TooltipTheme,
    TooltipThemeWith,
};

/// Properties for the [`Tooltip`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// Theme override.
    pub theme: Option<TooltipThemeWith>,
    /// Text to show in the [Tooltip].
    pub text: String,
}

/// `Tooltip` component. Use in combination with [TooltipContainer()].
///
/// # Styling
/// Inherits the [`TooltipTheme`](freya_hooks::TooltipTheme)
/// 
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         TooltipContainer {
///             tooltip: rsx!(
///                 Tooltip {
///                     text: "Hey!"
///                 }
///             ),
///             label { "Hover me!" }
///        }
///     )
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc_with_utils(|| {
/// #   rsx!(
/// #       Preview {
/// #           {app()}
/// #       }
/// #   )
/// # }, (185., 185.).into(), |mut utils| async move {
/// #   utils.wait_for_update().await;
/// #   utils.move_cursor((90., 90.)).await;
/// #   utils.wait_for_update().await;
/// #   utils.save_snapshot("./images/gallery_tooltip.png");
/// # });
/// ```
///
/// # Preview
/// ![Tooltip Preview][tooltip]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("tooltip", "images/gallery_tooltip.png")
)]
#[allow(non_snake_case)]
pub fn Tooltip(TooltipProps { text, theme }: TooltipProps) -> Element {
    let theme = use_applied_theme!(&theme, tooltip);
    let TooltipTheme {
        background,
        color,
        border_fill,
    } = theme;

    rsx!(
        rect {
            padding: "4 10",
            shadow: "0 0 4 1 rgb(0, 0, 0, 0.1)",
            border: "1 inner {border_fill}",
            corner_radius: "8",
            background: "{background}",
            label { max_lines: "1", font_size: "14", color: "{color}", "{text}" }
        }
    )
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TooltipPosition {
    Besides,
    Below,
}

/// `TooltipContainer` component.
///
/// Provides a hoverable area where to show a [Tooltip].
///
/// # Example
#[component]
pub fn TooltipContainer(
    tooltip: Element,
    children: Element,
    #[props(default = TooltipPosition::Below, into)] position: TooltipPosition,
) -> Element {
    let mut is_hovering = use_signal(|| false);
    let (reference, size) = use_node_signal();

    let onmouseenter = move |_: MouseEvent| {
        is_hovering.set(true);
    };

    let onmouseleave = move |_: MouseEvent| {
        is_hovering.set(false);
    };

    let direction = match position {
        TooltipPosition::Below => "vertical",
        TooltipPosition::Besides => "horizontal",
    };

    rsx!(
        rect {
            direction,
            reference,
            onmouseenter,
            onmouseleave,
            {children},
            rect {
                height: "0",
                width: "0",
                layer: "-1500",
                if *is_hovering.read() {
                    match position {
                        TooltipPosition::Below => rsx!(
                            rect {
                                width: "{size.read().area.width()}",
                                cross_align: "center",
                                padding: "5 0 0 0",
                                {tooltip}
                            }
                        ),
                        TooltipPosition::Besides => rsx!(
                            rect {
                                height: "{size.read().area.height()}",
                                main_align: "center",
                                padding: "0 0 0 5",
                                {tooltip}
                            }
                        ),
                    }
                }
            }
        }
    )
}
