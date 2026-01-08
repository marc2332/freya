#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::{
    animation::*,
    prelude::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut selected = use_state(|| 2);

    let on_wheel = move |e: Event<WheelEventData>| {
        if e.delta_y > 0. {
            *selected.write() -= 1;
        } else {
            *selected.write() += 1;
        }

        match selected() {
            4 => selected.set(1),
            0 => selected.set(3),
            _ => {}
        }
    };

    rect()
        .on_wheel(on_wheel)
        .content(Content::Flex)
        .horizontal()
        .spacing(5.)
        .width(Size::fill())
        .padding(5.)
        .children_iter(
            [
                "https://images.dog.ceo/breeds/dachshund/dachshund-2033796_640.jpg",
                "https://images.dog.ceo/breeds/cavapoo/doggo4.jpg",
                "https://images.dog.ceo/breeds/wolfhound-irish/n02090721_3109.jpg",
            ]
            .iter()
            .enumerate()
            .map(|(i, url)| {
                Card::default()
                    .selected(i == selected() - 1)
                    .child(
                        ImageViewer::new(*url)
                            .aspect_ratio(AspectRatio::Max)
                            .image_cover(ImageCover::Center)
                            .width(Size::fill())
                            .height(Size::fill()),
                    )
                    .into()
            }),
        )
}

#[derive(Default, PartialEq)]
struct Card {
    selected: bool,
    children: Vec<Element>,
}

impl Card {
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl ChildrenExt for Card {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Component for Card {
    fn render(&self) -> impl IntoElement {
        let animations = use_animation_with_dependencies(&self.selected, move |conf, selected| {
            conf.on_change(OnChange::Rerun);
            conf.on_creation(OnCreation::Run);
            let (from, to) = if *selected { (1.0, 3.0) } else { (3.0, 1.0) };
            AnimNum::new(from, to)
                .time(250)
                .ease(Ease::Out)
                .function(Function::Expo)
        });

        let width = animations.get().value();

        rect()
            .corner_radius(16.)
            .height(Size::fill())
            .width(Size::flex(width))
            .overflow(Overflow::Clip)
            .children(self.children.clone())
    }
}
