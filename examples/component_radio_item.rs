use freya::prelude::*;

#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [90, 95, 99]))]
fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    let mut selected = use_state(|| false);
    rect()
        .center()
        .expanded()
        .spacing(8.)
        .child(
            Tile::new()
                .on_select(move |_| selected.toggle())
                .child(RadioItem::new().selected(selected()))
                .leading("Click to select"),
        )
        .child(
            Tile::new()
                .on_select(move |_| selected.toggle())
                .child(RadioItem::new().selected(selected()).size(64.))
                .child("Click to select"),
        )
        .child(
            Tile::new()
                .on_select(move |_| selected.toggle())
                .child(RadioItem::new().selected(selected()).size(128.)),
        )
        .into()
}
