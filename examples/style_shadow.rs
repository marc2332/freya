use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    rect()
        .expanded()
        .center()
        .horizontal()
        .spacing(16.)
        .child(
            rect()
                .height(Size::px(80.))
                .width(Size::px(80.))
                .shadow(Shadow::new().blur(8.).color((255, 0, 0)).inset()),
        )
        .child(
            rect()
                .height(Size::px(80.))
                .width(Size::px(80.))
                .shadow(Shadow::new().x(24.).y(24.).blur(8.).color((0, 0, 0, 0.3)))
                .shadow(
                    Shadow::new()
                        .x(-24.)
                        .y(-24.)
                        .blur(8.)
                        .color((0, 255, 0, 0.3)),
                ),
        )
        .into()
}
