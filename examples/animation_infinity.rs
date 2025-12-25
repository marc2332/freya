use freya::{
    animation::*,
    prelude::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let animation = use_animation(|conf| {
        conf.on_creation(OnCreation::Run);
        conf.on_finish(OnFinish::reverse());
        AnimColor::new((246, 240, 240), (205, 86, 86)).time(400)
    });

    rect()
        .background(&*animation.read())
        .width(Size::fill())
        .height(Size::fill())
        .main_align(Alignment::center())
        .cross_align(Alignment::center())
        .spacing(8.0)
}
