#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Scroll example", (400.0, 400.0));
}

fn app() -> Element {
    let scroll_controller = use_scroll_controller(|| ScrollConfig {
        default_vertical_position: ScrollPosition::End,
        ..Default::default()
    });

    rsx!(
        ScrollView {
            for section in 0..4 {
                label {
                    margin: "10",
                    height: "35",
                    font_size: "20",
                    main_align: "center",
                    "Section {section}"
                }
                ScrollView {
                    height: "auto",
                    direction: "horizontal",
                    scroll_controller,
                    rect {
                        spacing: "2",
                        for i in 0..5 {
                            rect {
                                key: "{i}",
                                height: "70",
                                width: "1400",
                                background: "rgb(235, 235, 235)",
                                padding: "10",
                                label { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis mollis ligula nibh, sit amet hendrerit turpis scelerisque a. Cras sed magna a neque pretium egestas eu nec eros. Integer a arcu vitae augue tempus laoreet. Suspendisse aliquet turpis sit amet quam porttitor ullamcorper. Vivamus tortor quam, facilisis in hendrerit ac, bibendum vel justo." }
                            }
                        }
                    }
                }
            }
        }
    )
}
