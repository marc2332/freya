use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(520., 440.)))
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .color(Color::WHITE)
        .child(
            rect()
                .position(Position::new_absolute().left(160.).top(160.))
                .width(Size::px(200.))
                .height(Size::px(120.))
                .background((58, 110, 190))
                .layer(1)
                .child("Layer 1: in front"),
        )
        .child(
            rect()
                .position(Position::new_absolute().left(100.).top(100.))
                .width(Size::px(200.))
                .height(Size::px(120.))
                .background((150, 80, 90))
                .child("Default layer"),
        )
        .child(
            rect()
                .position(Position::new_absolute().left(40.).top(40.))
                .width(Size::px(200.))
                .height(Size::px(120.))
                .background((50, 130, 100))
                .layer(-1)
                .child("Layer -1: below"),
        )
        .child(
            rect()
                .position(Position::new_absolute().left(220.).top(220.))
                .width(Size::px(200.))
                .height(Size::px(120.))
                .background((120, 80, 170))
                .layer(Layer::Overlay)
                .child("Overlay"),
        )
        .child(
            rect()
                .position(Position::new_absolute().left(280.).top(280.))
                .width(Size::px(200.))
                .height(Size::px(120.))
                .background((170, 110, 40))
                .layer(Layer::OverlayLevel(2))
                .child("Overlay level 2"),
        )
}
