use freya::{
    animation::*,
    prelude::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut animation = use_animation(|_| AnimColor::new((246, 240, 240), (205, 86, 86)).time(400));

    rect()
        .background(&*animation.read())
        .width(Size::fill())
        .height(Size::fill())
        .main_align(Alignment::center())
        .cross_align(Alignment::center())
        .spacing(8.0)
        .children([
            Button::new()
                .on_press(move |_| {
                    animation.start();
                })
                .child("Start")
                .into(),
            Button::new()
                .on_press(move |_| {
                    animation.reverse();
                })
                .child("Reverse")
                .into(),
        ])
}
