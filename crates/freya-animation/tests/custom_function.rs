use freya_animation::prelude::*;

#[test]
fn custom_function_receives_ease() {
    for (ease, expected) in [(Ease::In, 35.), (Ease::Out, 85.), (Ease::InOut, 60.)] {
        let mut animation = AnimNum::new(10., 110.)
            .time(100)
            .ease(ease)
            .function(Function::Fn(|progress, ease| match ease {
                Ease::In => progress * progress,
                Ease::Out => 1. - (1. - progress).powi(2),
                Ease::InOut => progress,
            }));

        animation.advance(50, AnimDirection::Forward);
        assert_eq!(animation.value(), expected);
    }
}
