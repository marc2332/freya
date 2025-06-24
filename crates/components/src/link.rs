use std::borrow::Cow;

use dioxus::prelude::*;
use freya_core::platform::MouseButton;
use freya_elements::{
    self as dioxus_elements,
    events::MouseEvent,
};
use freya_hooks::{
    use_applied_theme,
    LinkThemeWith,
};
use freya_router::prelude::{
    navigator,
    NavigationTarget,
};

use crate::{
    Tooltip,
    TooltipContainer,
};

/// Tooltip configuration for the [`Link()`] component.
#[derive(Clone, PartialEq)]
pub enum LinkTooltip {
    /// No tooltip at all.
    None,
    /// Default tooltip.
    ///
    /// - For a route, this is the same as [`None`](crate::LinkTooltip::None).
    /// - For a URL, this is the value of that URL.
    Default,
    /// Custom tooltip to always show.
    Custom(String),
}

/// Navigate to [freya-router] routes or external URLs using [`Link`](freya_router::components::Link()).
/// When using internal routes make sure the Link is descendant of a [`Router`](freya_router::components::Router) component.
///
/// # Styling
///
/// Inherits the [`LinkTheme`](freya_hooks::LinkTheme) theme.
///
/// # Example
///
/// With Freya Router:
///
/// ```rust
/// # use dioxus::prelude::*;
/// # use freya_router::prelude::*;
/// # use freya_elements as dioxus_elements;
/// # use freya_components::Link;
/// # #[derive(Routable, Clone)]
/// # #[rustfmt::skip]
/// # enum AppRouter {
/// #     #[route("/")]
/// #     Settings,
/// #     #[route("/..routes")]
/// #     NotFound
/// # }
/// # #[component]
/// # fn Settings() -> Element { rsx!(rect { })}
/// # #[component]
/// # fn NotFound() -> Element { rsx!(rect { })}
/// # fn link_example_good() -> Element {
/// rsx! {
///     Link {
///         to: AppRouter::Settings,
///         label { "App Settings" }
///     }
/// }
/// # }
/// ```
///
/// With external routes:
///
/// ```rust
/// # use dioxus::prelude::*;
/// # use freya_elements as dioxus_elements;
/// # use freya_components::Link;
/// # fn link_example_good() -> Element {
/// rsx! {
///     Link {
///         to: "https://crates.io/crates/freya",
///         label { "Freya crates.io" }
///     }
/// }
/// # }
/// ```
#[allow(non_snake_case)]
#[component]
pub fn Link(
    /// Theme override.
    #[props(optional)]
    theme: Option<LinkThemeWith>,
    /// The route or external URL string to navigate to.
    #[props(into)]
    to: NavigationTarget,
    /// Inner children for the Link.
    children: Element,
    /// This event will be fired if opening an external link fails.
    #[props(optional)]
    onerror: Option<EventHandler<()>>,
    /// A little text hint to show when hovering over the anchor.
    ///
    /// Setting this to [`None`] is the same as [`LinkTooltip::Default`].
    /// To remove the tooltip, set this to [`LinkTooltip::None`].
    #[props(optional)]
    tooltip: Option<LinkTooltip>,
) -> Element {
    let theme = use_applied_theme!(&theme, link);
    let mut is_hovering = use_signal(|| false);

    let url = if let NavigationTarget::External(ref url) = to {
        Some(url.clone())
    } else {
        None
    };

    let onmouseenter = move |_: MouseEvent| {
        is_hovering.set(true);
    };

    let onmouseleave = move |_: MouseEvent| {
        is_hovering.set(false);
    };

    let onclick = {
        to_owned![url, to];
        move |event: MouseEvent| {
            if !matches!(event.trigger_button, Some(MouseButton::Left)) {
                return;
            }

            // Open the url if there is any
            // otherwise change the freya router route
            if let Some(url) = &url {
                let res = open::that(url);

                if let (Err(_), Some(onerror)) = (res, onerror.as_ref()) {
                    onerror.call(());
                }

                // TODO(marc2332): Log unhandled errors
            } else {
                let router = navigator();
                router.push(to.clone());
            }
        }
    };

    let color = if *is_hovering.read() {
        theme.highlight_color
    } else {
        Cow::Borrowed("inherit")
    };

    let tooltip = match tooltip {
        None | Some(LinkTooltip::Default) => url.clone(),
        Some(LinkTooltip::None) => None,
        Some(LinkTooltip::Custom(str)) => Some(str),
    };

    let link = rsx! {
        rect {
            onmouseenter,
            onmouseleave,
            onclick,
            color: "{color}",
            {children}
        }
    };

    if let Some(tooltip) = tooltip {
        rsx!(
            TooltipContainer {
                tooltip: rsx!(
                    Tooltip {
                        text: tooltip
                    }
                ),
                {link}
            }
        )
    } else {
        link
    }
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_router::prelude::{
        Outlet,
        Routable,
        Router,
    };
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn link() {
        #[derive(Routable, Clone)]
        #[rustfmt::skip]
        enum Route {
            #[layout(Layout)]
            #[route("/")]
            Home,
            #[route("/somewhere")]
            Somewhere,
            #[route("/..routes")]
            NotFound
        }

        #[allow(non_snake_case)]
        #[component]
        fn NotFound() -> Element {
            rsx! {
                label {
                    "Not found"
                }
            }
        }

        #[allow(non_snake_case)]
        #[component]
        fn Home() -> Element {
            rsx! {
                label {
                    "Home"
                }
            }
        }

        #[allow(non_snake_case)]
        #[component]
        fn Somewhere() -> Element {
            rsx! {
                label {
                    "Somewhere"
                }
            }
        }

        #[allow(non_snake_case)]
        #[component]
        fn Layout() -> Element {
            rsx!(
                Link {
                    to: Route::Home,
                    Button {
                        label { "Home" }
                    }
                }
                Link {
                    to: Route::Somewhere,
                    Button {
                        label { "Somewhere" }
                    }
                }
                Outlet::<Route> {}
            )
        }

        fn link_app() -> Element {
            rsx!(Router::<Route> {})
        }

        let mut utils = launch_test(link_app);

        // Check route is Home
        assert_eq!(utils.root().get(2).get(0).text(), Some("Home"));

        // Go to the "Somewhere" route
        utils.click_cursor((10., 55.)).await;

        // Check route is Somewhere
        assert_eq!(utils.root().get(2).get(0).text(), Some("Somewhere"));

        // Go to the "Home" route again
        utils.click_cursor((10., 10.)).await;

        // Check route is Home
        assert_eq!(utils.root().get(2).get(0).text(), Some("Home"));
    }
}
