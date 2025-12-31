#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::{
    animation::*,
    prelude::*,
};

#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [90, 95, 99]))]
fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect().child(
        VirtualScrollView::new(|i, _| {
            AnimatedContainer {
                height: 70.,
                i,
                children: rect()
                    .width(Size::fill())
                    .height(Size::fill())
                    .padding(4.)
                    .corner_radius(8.)
                    .color((255, 255, 255))
                    .background((0, 119, 182))
                    .child(format!("Item {i}"))
                    .into(),
            }
            .into()
        })
        .length(300)
        .item_size(70.)
        .height(Size::percent(100.)),
    )
}

#[derive(PartialEq)]
struct AnimatedContainer {
    height: f32,
    i: usize,
    children: Element,
}

impl Render for AnimatedContainer {
    fn render(&self) -> impl IntoElement {
        let animation = use_animation(|conf| {
            conf.on_creation(OnCreation::Run);
            AnimNum::new(350., 0.)
                .time(500)
                .ease(Ease::InOut)
                .function(Function::Expo)
        });

        let pos = animation.get().value();

        rect()
            .offset_x(pos)
            .width(Size::fill())
            .height(Size::px(self.height))
            .padding(4.)
            .child(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        DiffKey::from(&self.i)
    }
}
