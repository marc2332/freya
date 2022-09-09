use dioxus::core::UiEvent;
use dioxus::events::MouseData;
use dioxus::prelude::*;
use elements_namespace as dioxus_elements;
use elements_namespace::AttributeValue;
use freya::launch;

fn main() {
    launch(app);
}

static RUST_LOGO: &[u8] = include_bytes!("./rust_logo.png");

fn app(cx: Scope) -> Element {
    let mut size = use_state(&cx, || 150);

    let onscroll = move |e: UiEvent<MouseData>| {
        let page = e.coordinates().page();
        if *size.get() >= 15 && page.y > 15.0 {
            return;
        }
        size += (page.y as i32) * 20;
    };

    cx.render(rsx!(rect {
        width: "100%",
        height: "100%",
        padding: "100",
        onscroll: onscroll,
        image {
            image_data: AttributeValue::Bytes(RUST_LOGO),
            width: "{size}",
            height: "{size}",
        }
    }))
}
