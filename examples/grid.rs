use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!(
        rect {
            spacing: "12 24",
            content: "grid",
            grid_columns: "100, 1w, auto",
            grid_rows: "1w, 1w",
            width: "fill",
            height: "fill",

            rect {
                width: "fill",
                height: "fill",
                grid_column: "0 / 2",
                background: "red",
                corner_radius: "32",
            }

            rect {
                width: "fill",
                height: "fill",
                grid_column: "1 / 1",
                background: "blue",
                corner_radius: "32",
            }

            rect {
                width: "300",
                height: "300",
                visible_width: "50%",
                visible_height: "50%",
                grid_column: "2 / 1",
                background: "green",
                corner_radius: "32",
            }

            rect {
                width: "fill-min",
                height: "fill-min",
                grid_column: "2 / 1",
                grid_row: "1 / 1",
                background: "green",
                corner_radius: "32",
            }
        }
    )
}
