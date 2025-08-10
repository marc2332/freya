use std::sync::Arc;

use dioxus::prelude::*;
use freya_elements as dioxus_elements;

use crate::NativeContainer;

#[derive(Props, Clone)]
pub struct FreyaAppProps {
    pub app: Arc<dyn Fn() -> Element>,
}

impl PartialEq for FreyaAppProps {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[allow(non_snake_case)]
pub fn FreyaApp(props: FreyaAppProps) -> Element {
    #[allow(non_snake_case)]
    let App = props.app;

    let handle_error = |e: ErrorContext| {
        for error in e.errors().iter() {
            println!("{:?}", error);
        }

        #[cfg(not(debug_assertions))]
        std::process::exit(1);

        #[cfg(debug_assertions)]
        rsx!(
            rect {
                width: "fill",
                height: "fill",
                background: "rgb(138, 0, 0)",
                color: "white",
                main_align: "center",
                cross_align: "center",
                label {
                    "An unhandled error was thrown, check your logs."
                }
            }
        )
    };

    rsx!(
        NativeContainer {
            ErrorBoundary {
                handle_error,
                {App()}
            }
        }
    )
}
