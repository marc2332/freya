use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_router::*;
use freya_elements as dioxus_elements;

#[derive(Props)]
pub struct RouterLinkProps<'a> {
    pub to: &'a str,
    pub children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn RouterLink<'a>(cx: Scope<'a, RouterLinkProps<'a>>) -> Element<'a> {
    let svc = cx.use_hook(|| cx.consume_context::<Arc<RouterCore>>());

    render!(
        container {
            width: "100%",
            height: "100%",
            onclick: move |_| {
                if let Some(service) = svc {
                    service.push_route(cx.props.to, None, None);
                }
            },
            &cx.props.children
        }
    )
}
