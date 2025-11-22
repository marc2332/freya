use freya::prelude::*;

#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [90, 95, 99]))]
fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut checked = use_state(|| false);
    rect()
        .center()
        .expanded()
        .spacing(8.)
        .child(
            Tile::new()
                .on_select(move |_| checked.toggle())
                .child(Checkbox::new().selected(checked()))
                .leading("Click to check"),
        )
        .child(
            Tile::new()
                .on_select(move |_| checked.toggle())
                .child(Checkbox::new().selected(checked()).size(64.))
                .child("Click to check"),
        )
        .child(
            Tile::new()
                .on_select(move |_| checked.toggle())
                .child(Checkbox::new().selected(checked()).size(128.)),
        )
}
