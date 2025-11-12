use freya::{
    animation::*,
    prelude::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    let mut animation = use_animation(|_| {
        AnimNum::new(50., 550.)
            .function(Function::Elastic)
            .ease(Ease::Out)
            .time(1500)
    });

    let value = animation.read().value();

    rect()
        .children([
            rect()
                .position(Position::new_absolute().left(value).top(50.))
                .background(Color::BLUE)
                .width(Size::px(100.))
                .height(Size::px(100.))
                .into(),
            rect()
                .horizontal()
                .width(Size::fill())
                .height(Size::fill())
                .main_align(Alignment::center())
                .cross_align(Alignment::center())
                .spacing(8.0)
                .child(
                    Button::new()
                        .on_press(move |_| {
                            animation.start();
                        })
                        .child("Start"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| {
                            animation.reverse();
                        })
                        .child("Reverse"),
                )
                .into(),
        ])
        .into()
}
