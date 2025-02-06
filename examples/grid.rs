use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!(
        rect {
            spacing: "12",
            content: "grid",
            grid_columns: "100, 1*, 1*",
            grid_rows: "1*",
            width: "fill",
            height: "fill",

            rect {
                width: "grid(0, 1)",
                height: "grid(0, 1)",
                background: "red",
                corner_radius: "32",
            }

            rect {
                width: "grid(1, 1)",
                height: "grid(0, 1)",
                background: "blue",
                corner_radius: "32",
            }

            rect {
                width: "grid(2, 1)",
                height: "grid(0, 1)",
                background: "green",
                corner_radius: "32",
            }
        }
    )
}
