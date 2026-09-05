use freya::{
    animation::*,
    prelude::*,
};

use crate::showcases::heading;

#[derive(PartialEq)]
pub struct GraphicsShowcase;

impl Component for GraphicsShowcase {
    fn render(&self) -> impl IntoElement {
        let spin = use_animation(|conf| {
            conf.on_creation(OnCreation::Run);
            conf.on_finish(OnFinish::restart());
            AnimNum::new(0., 360.).time(6000).function(Function::Linear)
        });

        rect()
            .spacing(20.)
            .child(heading("Graphics", "Shapes, colors and rotation"))
            .child(
                rect()
                    .horizontal()
                    .spacing(16.)
                    .cross_align(Alignment::center())
                    .child(
                        rect()
                            .width(Size::px(120.))
                            .height(Size::px(120.))
                            .corner_radius(24.)
                            .background(
                                LinearGradient::new()
                                    .angle(140.)
                                    .stop(((236, 72, 153), 0.))
                                    .stop(((99, 102, 241), 100.)),
                            )
                            .shadow((0., 10., 24., 0., (0, 0, 0, 60))),
                    )
                    .child(
                        rect()
                            .width(Size::px(120.))
                            .height(Size::px(120.))
                            .corner_radius(60.)
                            .background(
                                RadialGradient::new()
                                    .stop(((250, 204, 21), 0.))
                                    .stop(((239, 68, 68), 100.)),
                            ),
                    )
                    .child(
                        rect()
                            .width(Size::px(100.))
                            .height(Size::px(100.))
                            .corner_radius(16.)
                            .background((16, 185, 129))
                            .rotate(spin.get().value())
                            .center()
                            .color((255, 255, 255))
                            .child("spin"),
                    ),
            )
            .child(
                rect()
                    .width(Size::fill())
                    .height(Size::px(90.))
                    .corner_radius(16.)
                    .background(
                        LinearGradient::new()
                            .angle(90.)
                            .stop(((15, 23, 42), 0.))
                            .stop(((59, 130, 246), 100.)),
                    )
                    .center()
                    .color((255, 255, 255))
                    .font_size(20.)
                    .child("github.com/marc2332/freya"),
            )
    }
}
