#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
	launch_with_props(app, "Controlled Example", (600.0, 400.0));
}

fn app() -> Element {
	let mut scroll_controller = use_scroll_controller(|| ScrollConfig {
		default_vertical_position: ScrollPosition::Start,
		..Default::default()
	});

	rsx!(
        rect {
            height: "fill",
            width: "fill",
            ScrollView {
                reverse: true,
                scroll_controller,
                theme: theme_with!(ScrollViewTheme {
                    width: "100%%".into(),
					height: "100%".into(),
                }),
                Card {}
                Card {}
                Card {}
            }
        }
    )
}

#[component]
fn Card() -> Element {
	rsx!(
        rect {
            border: "15 solid rgb(43,106,208)",
            height: "220",
			width: "500",
            background: "white",
            padding: "25",
            label {  "Scroll..." }
        }
    )
}
