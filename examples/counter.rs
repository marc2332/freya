use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        rect {
            width: "fill",
            height: "fill",
            main_align: "center",
            cross_align: "center",

            rect {
                width: "200",
                height: "200",
                corner_radius: "12",
                overflow: "clip",

                // Should be clipped by a rounded rectangle
                rect {
                    width: "fill",
                    height: "fill",
                    background: "green"
                }
            }
        }
    }
}
