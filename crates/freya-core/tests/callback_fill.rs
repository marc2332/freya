use freya::prelude::*;
use freya_engine::prelude::Paint;
use freya_testing::prelude::*;

fn render_fill(fill: Fill) -> Vec<u8> {
    let (mut runner, ()) = TestingRunner::new(
        move || {
            rect()
                .padding(20.)
                .spacing(10.)
                .font_size(24.)
                .color(fill.clone())
                .child(
                    rect()
                        .width(Size::px(180.))
                        .height(Size::px(30.))
                        .background(fill.clone()),
                )
                .child(label().width(Size::px(250.)).text("Callback label"))
                .child(
                    paragraph()
                        .width(Size::px(250.))
                        .span("Inherited ")
                        .span(Span::new("solid").color(Color::GREEN)),
                )
                .child(
                    paragraph()
                        .width(Size::px(250.))
                        .color(Color::GREEN)
                        .span("Solid ")
                        .span(Span::new("callback").color(fill.clone())),
                )
        },
        (600., 500.).into(),
        |_| (),
        1.,
    );
    runner.sync_and_update();
    runner.render().as_bytes().to_vec()
}

#[test]
fn callback_paint_matches_gradient_fill() {
    let gradient = LinearGradient::new()
        .angle(90.)
        .stop((Color::RED, 0.))
        .stop((Color::BLUE, 100.));
    let callback_gradient = gradient.clone();
    let callback = RenderCallback::new(move |context| {
        if context.size.width <= 0. || context.size.height <= 0. {
            return;
        }

        let origin = match &context.target {
            RenderTarget::Background { .. } => freya::torin::geometry::Point2D::zero(),
            RenderTarget::TextStyle(_) => context.origin,
        };
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_shader(callback_gradient.prepare_shader(Area::new(origin, context.size)));
        context.set_paint(&paint);
    });

    assert!(
        render_fill(callback.into()) == render_fill(gradient.into()),
        "callback paint differs from the built-in gradient",
    );
}
