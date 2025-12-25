use freya::{
    animation::*,
    prelude::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut toggle = use_state(|| false);
    let mut animations = use_animation(|_conf| {
        AnimSequential::new([
            AnimNum::new(0., 360.)
                .time(400)
                .ease(Ease::InOut)
                .function(Function::Expo),
            AnimNum::new(0., 180.)
                .time(1000)
                .ease(Ease::Out)
                .function(Function::Elastic),
        ])
    });

    let sequential = animations.get();

    let rotate_a = sequential[0].value();
    let rotate_b = sequential[1].value();

    rect()
        .expanded()
        .center()
        .spacing(50.0)
        .horizontal()
        .on_press(move |_| {
            if toggle.toggled() {
                animations.start();
            } else {
                animations.reverse();
            }
        })
        .child(
            rect()
                .width(Size::px(100.))
                .height(Size::px(100.))
                .rotate(rotate_a)
                .background((0, 119, 182)),
        )
        .child(
            rect()
                .width(Size::px(100.))
                .height(Size::px(100.))
                .rotate(rotate_b)
                .background((0, 119, 182)),
        )
}
