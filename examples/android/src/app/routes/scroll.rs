use freya::{
    animation::*,
    prelude::*,
};

#[derive(PartialEq)]
pub struct ScrollViewDemo;

impl Component for ScrollViewDemo {
    fn render(&self) -> impl IntoElement {
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
        .length(300usize)
        .item_size(70.)
        .height(Size::percent(100.))
    }
}

#[derive(PartialEq)]
struct AnimatedContainer {
    height: f32,
    i: usize,
    children: Element,
}

impl Component for AnimatedContainer {
    fn render(&self) -> impl IntoElement {
        let animation = use_animation(|conf| {
            conf.on_creation(OnCreation::Run);
            AnimNum::new(350., 0.)
                .time(500)
                .ease(Ease::InOut)
                .function(Function::Expo)
        });

        rect()
            .offset_x(animation.get().value())
            .width(Size::fill())
            .height(Size::px(self.height))
            .padding(4.)
            .child(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        DiffKey::from(&self.i)
    }
}
