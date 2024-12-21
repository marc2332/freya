use std::time::Duration;

use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use freya_hooks::{
    use_animation_with_dependencies,
    use_node_signal_with_prev,
    AnimDirection,
    AnimNum,
    Ease,
    Function,
};

#[component]
pub fn AnimatedPosition(
    children: Element,
    width: String,
    height: String,
    #[props(default = Function::default())] function: Function,
    #[props(default = Duration::from_millis(250))] duration: Duration,
    #[props(default = Ease::default())] ease: Ease,
) -> Element {
    let mut render_element = use_signal(|| false);
    let (reference, size, old_size) = use_node_signal_with_prev();

    let animations = use_animation_with_dependencies(
        &(function, duration, ease),
        move |ctx, (function, duration, ease)| {
            let old_size = old_size().unwrap_or_default();
            let size = size().unwrap_or_default();
            (
                ctx.with(
                    AnimNum::new(size.area.origin.x, old_size.area.origin.x)
                        .duration(duration)
                        .ease(ease)
                        .function(function),
                ),
                ctx.with(
                    AnimNum::new(size.area.origin.y, old_size.area.origin.y)
                        .duration(duration)
                        .ease(ease)
                        .function(function),
                ),
            )
        },
    );

    use_effect(move || {
        if animations.is_running() {
            render_element.set(true);
        }
    });

    use_effect(move || {
        let has_size = size.read().is_some();
        let has_old_size = old_size.read().is_some();
        if has_size && has_old_size {
            animations.run(AnimDirection::Reverse);
        } else if has_size {
            render_element.set(true);
        }
    });

    let (offset_x, offset_y) = animations.get();
    let offset_x = offset_x.read().as_f32();
    let offset_y = offset_y.read().as_f32();

    rsx!(
        rect {
            reference,
            width: "{width}",
            height: "{height}",
            rect {
                width: "0",
                height: "0",
                offset_x: "{offset_x}",
                offset_y: "{offset_y}",
                position: "global",
                if render_element() {
                    rect {
                        width: "{size.read().as_ref().unwrap().area.width()}",
                        height: "{size.read().as_ref().unwrap().area.height()}",
                        {children}
                    }
                }
            }
        }
    )
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn animated_position() {
        fn animated_position_app() -> Element {
            let mut padding = use_signal(|| (100., 100.));

            rsx!(
                rect {
                    padding: "{padding().0} {padding().1}",
                    onclick: move |_| {
                        padding.write().0 += 10.;
                        padding.write().1 += 10.;
                    },
                    AnimatedPosition {
                        width: "50",
                        height: "50",
                        function: Function::Linear
                    }
                }
            )
        }

        let mut utils = launch_test(animated_position_app);

        // Disable event loop ticker
        utils.config().event_loop_ticker = false;

        let root = utils.root();
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        let get_positions = || {
            root.get(0)
                .get(0)
                .get(0)
                .get(0)
                .layout()
                .unwrap()
                .area
                .origin
        };

        assert_eq!(get_positions().x, 100.);
        assert_eq!(get_positions().y, 100.);

        utils.click_cursor((5.0, 5.0)).await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        tokio::time::sleep(Duration::from_millis(125)).await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        assert!(get_positions().x < 106.);
        assert!(get_positions().x > 105.);

        assert!(get_positions().y < 106.);
        assert!(get_positions().y > 105.);

        utils.config().event_loop_ticker = true;

        utils.wait_for_update().await;
        tokio::time::sleep(Duration::from_millis(125)).await;
        utils.wait_for_update().await;

        assert_eq!(get_positions().x, 110.);
    }
}
