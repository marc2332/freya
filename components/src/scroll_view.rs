use dioxus::{core::UiEvent, events::WheelData, prelude::*};
use elements_namespace as dioxus_elements;

#[allow(non_snake_case)]
pub fn ScrollView<'a>(cx: Scope<'a, ScrollViewProps<'a>>) -> Element {
    let mut y = use_state(&cx, || 0);

    let onwheel = move |e: UiEvent<WheelData>| {
        let wheel_y = e.delta().strip_units().y;
        if *y.get() >= 0 && wheel_y > 0.0 {
            return;
        }
        y += (wheel_y as i32) * 20;
    };

    let width = cx.props.width.unwrap_or("100%");
    let height = cx.props.height.unwrap_or("100%");
    let padding = cx.props.padding.unwrap_or("0");

    cx.render(rsx!(
        container {
            padding: "{padding}",
            width: "{width}",
            height: "{height}",
            scroll_y: "{y}",
            onwheel: onwheel,
            &cx.props.children
        }
    ))
}

#[derive(Props)]
pub struct ScrollViewProps<'a> {
    children: Element<'a>,
    #[props(optional)]
    height: Option<&'a str>,
    #[props(optional)]
    width: Option<&'a str>,
    #[props(optional)]
    padding: Option<&'a str>,
}
