use crate::Tooltip;
use dioxus::prelude::*;
use dioxus_router::prelude::{navigator, IntoRoutable};
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::{use_applied_theme, LinkThemeWith};
use std::borrow::Cow;
use winit::event::MouseButton;

/// Tooltip configuration for the [`Link`] component.
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

/// Similar to [`Link`](dioxus_router::components::Link), but you can use it in Freya.
/// Both internal routes (dioxus-router) and external links are supported. When using internal routes
/// make sure the Link is descendant of a [`Router`](dioxus_router::components::Router) component.
///
/// # Styling
///
/// Inherits the [`LinkTheme`](freya_hooks::LinkTheme) theme.
///
/// # Example
///
/// With Dioxus Router:
///
/// ```rust
/// # use dioxus::prelude::*;
/// # use dioxus_router::prelude::*;
/// # use freya_elements::elements as dioxus_elements;
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
/// # use freya_elements::elements as dioxus_elements;
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
    to: IntoRoutable,
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

    let url = if let IntoRoutable::FromStr(ref url) = to {
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
            // otherwise change the dioxus router route
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

    let main_rect = rsx! {
        rect {
            onmouseenter,
            onmouseleave,
            onclick,
            color: "{color}",
            {children}
        }
    };

    let Some(tooltip) = tooltip else {
        return rsx!({ main_rect });
    };

    rsx! {
        {main_rect}
        rect {
            height: "0",
            layer: "-999",
            rect {
                width: "100v",
                if *is_hovering.read() {
                    Tooltip {
                        url: tooltip
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use dioxus_router::prelude::{Outlet, Routable, Router};
    use freya::prelude::*;
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

        utils.wait_for_update().await;
        utils.wait_for_update().await;

        // Check route is Home
        assert_eq!(utils.root().get(2).get(0).text(), Some("Home"));

        // Go to the "Somewhere" route
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (5., 70.).into(),
            button: Some(MouseButton::Left),
        });

        utils.wait_for_update().await;
        utils.wait_for_update().await;

        // Check route is Somewhere
        assert_eq!(utils.root().get(2).get(0).text(), Some("Somewhere"));

        // Go to the "Home" route again
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (5., 5.).into(),
            button: Some(MouseButton::Left),
        });

        utils.wait_for_update().await;
    }
}
