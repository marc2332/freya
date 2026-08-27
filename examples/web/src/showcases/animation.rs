use freya::{
    animation::*,
    prelude::*,
};

#[derive(PartialEq)]
pub struct AnimationShowcase;

impl Component for AnimationShowcase {
    fn render(&self) -> impl IntoElement {
        let mut expanded = use_state(|| false);

        let mut animation = use_animation(|conf| {
            conf.on_creation(OnCreation::Nothing);
            (
                AnimNum::new(0., 1.)
                    .time(600)
                    .ease(Ease::Out)
                    .function(Function::Expo),
                AnimColor::new((59, 130, 246), (236, 72, 153))
                    .time(600)
                    .ease(Ease::Out)
                    .function(Function::Expo),
            )
        });

        let (progress, color) = animation.get().value();

        rect()
            .spacing(20.)
            .child(super::heading(
                "Animation",
                "Press the button and watch it take its time",
            ))
            .child(
                Button::new()
                    .on_press(move |_| {
                        expanded.toggle();
                        if expanded() {
                            animation.start();
                        } else {
                            animation.reverse();
                        }
                    })
                    .child(if expanded() { "Collapse" } else { "Expand" }),
            )
            .child(
                rect()
                    .height(Size::px(64.))
                    .width(Size::percent(20. + progress * 70.))
                    .corner_radius(16.)
                    .background(color)
                    .center()
                    .color((255, 255, 255))
                    .child(format!("{:.0}%", progress * 100.)),
            )
    }
}
