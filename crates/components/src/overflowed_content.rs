use std::time::Duration;

use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{
    use_animation,
    use_node_signal,
    AnimDirection,
    AnimNum,
    Ease,
    Function,
    OnFinish,
};

/// Animate the content of a container when the content overflows.
///
/// This is primarily targeted to text that can't be fully shown in small layouts.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         Button {
///             OverflowedContent {
///                 width: "100",
///                 rect {
///                     direction: "horizontal",
///                     cross_align: "center",
///                     label {
///                         "Freya is a cross-platform GUI library for Rust"
///                     }
///                 }
///             }
///         }
///     )
/// }
/// ```
#[component]
pub fn OverflowedContent(
    children: Element,
    #[props(default = "100%".to_string())] width: String,
    #[props(default = "auto".to_string())] height: String,
    #[props(default = Duration::from_secs(4))] duration: Duration,
) -> Element {
    let (label_reference, label_size) = use_node_signal();
    let (rect_reference, rect_size) = use_node_signal();

    let rect_width = rect_size.read().area.width();
    let label_width = label_size.read().area.width();
    let does_overflow = label_width > rect_width;

    let animation = use_animation(move |conf| {
        conf.on_finish(OnFinish::Restart);

        AnimNum::new(0., 100.)
            .duration(duration)
            .ease(Ease::InOut)
            .function(Function::Linear)
    });

    use_effect(use_reactive!(|does_overflow| {
        if does_overflow {
            animation.run(AnimDirection::Forward);
        }
    }));

    let progress = animation.get().read().read();
    let offset_x = if does_overflow {
        ((label_width + rect_width) * progress / 100.) - rect_width
    } else {
        0.
    };

    rsx!(
        rect {
            width,
            height,
            offset_x: "{-offset_x}",
            overflow: "clip",
            reference: rect_reference,
            rect {
                reference: label_reference,
                max_lines: "1",
                {children}
            }
        }
    )
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use freya::prelude::*;
    use freya_testing::prelude::*;
    use tokio::time::sleep;

    #[tokio::test]
    pub async fn overflowed_content() {
        fn app() -> Element {
            rsx!(
                OverflowedContent {
                    duration: Duration::from_millis(50),
                    width: "50",
                    label {
                        "123456789123456789"
                    }
                }
            )
        }

        let mut utils = launch_test(app);

        // Disable event loop ticker
        utils.config().event_loop_ticker = false;

        let root = utils.root();
        let label = root.get(0).get(0).get(0);

        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        assert_eq!(label.layout().unwrap().area.min_x(), 50.);

        sleep(Duration::from_millis(25)).await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        assert_ne!(label.layout().unwrap().area.min_x(), 50.);
    }
}
