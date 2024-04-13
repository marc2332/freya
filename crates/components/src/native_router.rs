use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{MouseButton, PointerEvent, PointerType};

/// Provides native functionalities for a dioxus-router.
///
/// Features:
/// - Navigate using back and forward buttons of the mouse.
///
#[allow(non_snake_case)]
#[component]
pub fn NativeRouter(children: Element) -> Element {
    let router = use_navigator();

    let onpointerup = move |e: PointerEvent| {
        if let PointerType::Mouse { trigger_button } = e.data().get_pointer_type() {
            match trigger_button {
                Some(MouseButton::Back) => router.go_back(),
                Some(MouseButton::Forward) => router.go_forward(),
                _ => {}
            }
        }
    };

    rsx!(
        rect {
            onpointerup,
            {children}
        }
    )
}

#[cfg(test)]
mod test {
    use dioxus_router::prelude::{Outlet, Routable, Router};
    use freya::prelude::*;
    use freya_core::events::{EventName, PlatformEvent};
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn native_router() {
        #[derive(Routable, Clone, PartialEq)]
        #[rustfmt::skip]
        pub enum Route {
            #[layout(App)]
                #[route("/")]
                A,
                #[route("/B")]
                B,
            #[end_layout]
            #[route("/..route")]
            NotFound { },
        }

        #[allow(non_snake_case)]
        #[component]
        fn NotFound() -> Element {
            rsx!(
                label {
                    "NotFound"
                }
            )
        }

        #[allow(non_snake_case)]
        #[component]
        fn A() -> Element {
            rsx!(
                label {
                    "A"
                }
            )
        }

        #[allow(non_snake_case)]
        #[component]
        fn B() -> Element {
            rsx!(
                label {
                    "B"
                }
            )
        }

        #[allow(non_snake_case)]
        fn App() -> Element {
            rsx!(
                NativeRouter {
                    Link {
                        to: Route::B,
                        label {
                            "Got to B"
                        }
                    },
                    Outlet::<Route> {  }
                }
            )
        }

        let mut utils = launch_test(|| -> Element { rsx!(Router::<Route> {}) });

        assert_eq!(utils.root().get(0).get(1).get(0).text(), Some("A"));

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (5.0, 5.0).into(),
            button: Some(MouseButton::Left),
        });

        utils.wait_for_update().await;

        assert_eq!(utils.root().get(0).get(1).get(0).text(), Some("B"));

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::PointerUp,
            cursor: (5.0, 5.0).into(),
            button: Some(MouseButton::Back),
        });

        utils.wait_for_update().await;

        assert_eq!(utils.root().get(0).get(1).get(0).text(), Some("A"));

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::PointerUp,
            cursor: (5.0, 5.0).into(),
            button: Some(MouseButton::Forward),
        });

        utils.wait_for_update().await;

        assert_eq!(utils.root().get(0).get(1).get(0).text(), Some("B"));
    }
}
