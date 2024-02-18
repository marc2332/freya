use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;

use freya_hooks::{use_applied_theme, ExternalLinkThemeWith};

use crate::Tooltip;

/// [`ExternalLink`] component properties.
#[derive(Props, Clone, PartialEq)]
pub struct ExternalLinkProps {
    /// Theme override.
    pub theme: Option<ExternalLinkThemeWith>,
    /// Inner children for the ExternalLink.
    pub children: Element,
    /// Handler for the `onerror` event.
    pub onerror: Option<EventHandler<()>>,
    /// Whether to show a tooltip with the URL or not.
    #[props(optional, default = true)]
    pub show_tooltip: bool,
    /// The ExternalLink destination URL.
    #[props(into)]
    pub url: String,
}

/// `Link` for external locations, e.g websites.
///
/// # Props
/// See [`ExternalLinkProps`].
///
/// # Styling
/// Inherits the [`ExternalLinkTheme`](freya_hooks::ExternalLinkTheme) theme.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         ExternalLink {
///             url: "https://github.com",
///             label {
///                 "GitHub"
///             }
///         }
///     )
/// }
/// ```
///
#[allow(non_snake_case)]
pub fn ExternalLink(props: ExternalLinkProps) -> Element {
    let theme = use_applied_theme!(&props.theme, external_link);
    let mut is_hovering = use_signal(|| false);

    let onmouseover = move |_: MouseEvent| {
        *is_hovering.write() = true;
    };

    let onmouseleave = move |_: MouseEvent| {
        *is_hovering.write() = false;
    };

    let onclick = {
        let url = props.url.clone();
        move |_: MouseEvent| {
            let res = open::that(url.clone());
            if let (Err(_), Some(onerror)) = (res, props.onerror.as_ref()) {
                onerror.call(());
            }
            // TODO(marc2332): Log unhandled errors
        }
    };

    let color = if *is_hovering.read() {
        theme.highlight_color.as_ref()
    } else {
        "inherit"
    };

    rsx!(
        rect {
            onmouseover,
            onmouseleave,
            onclick,
            color: "{color}",
            {props.children}
        }
        rect {
            height: "0",
            width: "0",
            layer: "-999",
            rect {
                width: "100v",
                if *is_hovering.read() && props.show_tooltip {
                    Tooltip {
                        url: props.url.clone()
                    }
                }
            }
        }
    )
}
