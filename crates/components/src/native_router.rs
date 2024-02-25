use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{PointerType, MouseButton, PointerEvent};

#[allow(non_snake_case)]
#[component]
pub fn NativeRouter() -> Element {
    let router = use_navigator();

    let onglobalpointerup = move |e: PointerEvent| {
        match e.data().get_pointer_type() {
            PointerType::Mouse { trigger_button } => {
                match trigger_button {
                    Some(MouseButton::Back) => {
                        router.go_back()
                    },
                    Some(MouseButton::Forward) => {
                        router.go_forward()
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    };

    rsx!(
        rect {
            onglobalpointerup,
        }
    )
}