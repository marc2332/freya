use dioxus::prelude::*;
use dioxus_router::use_router;
use freya_elements as dioxus_elements;

/// Properties for the Router Link component.
#[derive(Props)]
pub struct RouterLinkProps<'a> {
    pub to: &'a str,
    pub children: Element<'a>,
}

/// Freya Link for Dioxus Router.
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
