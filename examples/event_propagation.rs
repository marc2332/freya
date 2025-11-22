use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect()
        .width(Size::px(200.))
        .height(Size::px(200.))
        .background(Color::RED)
        .on_press(|_| println!("clicked 1"))
        .child(
            rect()
                .width(Size::px(150.))
                .height(Size::px(150.))
                .background(Color::BLUE)
                .on_press(|_| println!("clicked 2"))
                .child(
                    rect()
                        .width(Size::px(100.))
                        .height(Size::px(100.))
                        .background(Color::GREEN)
                        .on_press(|e: Event<PressEventData>| {
                            e.stop_propagation();
                            println!("clicked 3")
                        }),
                ),
        )
}
