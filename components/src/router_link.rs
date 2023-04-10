use dioxus::prelude::*;
use dioxus_router::use_router;
use freya_elements::elements as dioxus_elements;

/// [`RouterLink`] component properties.
#[derive(Props)]
pub struct RouterLinkProps<'a> {
    // The RouterLink destination.
    pub to: &'a str,
    /// Inner children for the RouterLink.
    pub children: Element<'a>,
}

/// `Link` for Dioxus Router.
///
/// # Props
/// See [`RouterLinkProps`]
///
#[allow(non_snake_case)]
pub fn RouterLink<'a>(cx: Scope<'a, RouterLinkProps<'a>>) -> Element<'a> {
    let svc = use_router(cx);

    render!(
        container {
            width: "100%",
            height: "100%",
            onclick: move |_| {
                svc.push_route(cx.props.to, None, None);
            },
            &cx.props.children
        }
    )
}
