use std::time::Duration;

use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use tokio::time::sleep;

/// [`Loader`] component properties.
#[derive(Props, PartialEq)]
pub struct LoaderProps {
    
}

/// `Loader` component.
///
/// # Props
/// See [`LoaderProps`].
///
/// # Styling
///
#[allow(non_snake_case)]
pub fn Loader(cx: Scope<LoaderProps>) -> Element {
    let degrees = use_state(cx,|| 0);

    use_effect(cx, (), move |_| {
        to_owned![degrees];
        async move {
            loop {
                sleep(Duration::from_millis(1)).await;
                if *degrees.get() > 360 {
                    degrees.set(0);
                } else {
                    degrees += 3;
                }
            }
        }
    });

    render!(
        svg {
            rotate: "{degrees}",
            width: "31",
            height: "31",
            svg_content: r#"
                <svg width="31" height="31" viewBox="0 0 31 31" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M15.5235 27.6652C22.2292 27.6652 27.6652 22.2292 27.6652 15.5235C27.6652 8.81783 22.2292 3.38182 15.5235 3.38182C8.81783 3.38182 3.38182 8.81783 3.38182 15.5235C3.38182 22.2292 8.81783 27.6652 15.5235 27.6652Z" stroke="white" stroke-opacity="0.5" stroke-width="4"/>
                    <path d="M27.6652 15.5235C27.6652 8.81859 22.2284 3.38182 15.5235 3.38182" stroke="white" stroke-width="4"/>
                </svg>
            "#
        }
    )
}
