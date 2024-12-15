use std::time::Duration;

use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{
    use_animation,
    use_node_signal,
    AnimNum,
    Ease,
    Function,
    OnFinish,
};

#[component]
pub fn OverflowedContent(
    children: Element,
    #[props(default = "100%".to_string())] width: String,
    #[props(default = "auto".to_string())] height: String,
    #[props(default = Duration::from_secs(4))] duration: Duration,
) -> Element {
    let (label_reference, label_size) = use_node_signal();
    let (rect_reference, rect_size) = use_node_signal();
    let animations = use_animation(move |ctx| {
        ctx.auto_start(true);
        ctx.on_finish(OnFinish::Restart);
        (ctx.with(
            AnimNum::new(0., 100.)
                .time(duration.as_millis() as u64)
                .ease(Ease::InOut)
                .function(Function::Linear),
        ),)
    });

    let (progress,) = animations.get();
    let progress = progress.read().as_f32();
    let rect_width = rect_size.read().area.width();
    let label_width = label_size.read().area.width();
    let offset_x = ((label_width + rect_width) * progress / 100.) - rect_width;

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
