use std::time::Duration;

use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
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
                position: "fixed",
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
