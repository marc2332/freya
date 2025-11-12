use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    rect()
        .width(Size::px(200.))
        .height(Size::px(200.))
        .background(Color::RED)
        .on_press(|_| println!("clicked 1"))
        .children([rect()
            .width(Size::px(150.))
            .height(Size::px(150.))
            .background(Color::BLUE)
            .on_press(|_| println!("clicked 2"))
            .children([rect()
                .width(Size::px(100.))
                .height(Size::px(100.))
                .background(Color::GREEN)
                .on_press(|e: Event<PressEventData>| {
                    e.stop_propagation();
                    println!("clicked 3")
                })
                .into()])
            .into()])
        .into()
}
