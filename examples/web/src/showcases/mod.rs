macro_rules! demos {
    ($($name:ident => $title:literal $body:block)*) => {
        $(
            #[derive(PartialEq)]
            struct $name;

            impl Component for $name {
                fn render(&self) -> impl IntoElement $body
            }
        )*

        const DEMOS: &[crate::showcases::Demo] = &[$(crate::showcases::Demo {
            title: $title,
            render: || $name.into(),
        }),*];
    };
}

mod animation;
mod drag_drop;
mod effects;
mod gallery;
mod gif;
mod i18n;
mod markdown;
mod material;
mod scroll;

use freya::prelude::*;

pub use crate::showcases::{
    animation::AnimationShowcase,
    drag_drop::DragDropShowcase,
    effects::EffectsShowcase,
    gallery::GalleryShowcase,
    gif::GifShowcase,
    i18n::I18nShowcase,
    markdown::MarkdownShowcase,
    material::MaterialShowcase,
    scroll::ScrollShowcase,
};

const CARD_WIDTH: f32 = 300.;
const CARD_HEIGHT: f32 = 350.;
const GAP: f32 = 12.;

pub fn heading(title: &str, subtitle: &str) -> impl IntoElement {
    rect()
        .spacing(4.)
        .child(
            rect()
                .font_size(26.)
                .font_weight(FontWeight::BOLD)
                .child(title),
        )
        .child(rect().opacity(0.6).child(subtitle))
}

fn inline_menu() -> Rect {
    let colors = use_theme().read().colors.clone();
    let a11y_id = use_a11y();
    use_provide_context(move || MenuGroup { group_id: a11y_id });

    rect()
        .width(Size::px(180.))
        .padding(4.)
        .corner_radius(8.)
        .background(colors.background)
        .border(Border::new().width(1.).fill(colors.border))
}

pub struct Demo {
    pub title: &'static str,
    pub render: fn() -> Element,
}

impl PartialEq for Demo {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

impl Demo {
    fn card(&self) -> Card {
        Card::new()
            .padding(0.)
            .width(Size::px(CARD_WIDTH))
            .height(Size::px(CARD_HEIGHT))
            .child(
                rect()
                    .expanded()
                    .spacing(8.)
                    .child(
                        rect()
                            .padding((12., 12., 0., 12.))
                            .opacity(0.6)
                            .child(label().text(self.title).font_size(13.)),
                    )
                    .child(rect().expanded().center().child((self.render)())),
            )
    }
}

#[derive(PartialEq)]
pub struct DemoGrid {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub demos: &'static [Demo],
}

impl Component for DemoGrid {
    fn render(&self) -> impl IntoElement {
        let demos = self.demos;
        let mut available_width = use_state(|| CARD_WIDTH);
        let columns = (((available_width() + GAP) / (CARD_WIDTH + GAP)) as usize).max(1);

        rect()
            .expanded()
            .spacing(20.)
            .child(
                rect()
                    .padding((24., 24., 0., 24.))
                    .child(heading(self.title, self.subtitle)),
            )
            .child(
                rect()
                    .expanded()
                    .padding((0., 0., 0., 24.))
                    .on_sized(move |event: Event<SizedEventData>| {
                        available_width.set_if_modified(event.area.width())
                    })
                    .child(
                        VirtualScrollView::new_with_data(
                            (columns, demos),
                            |item, (columns, demos)| {
                                let start = item.index * columns;
                                let end = (start + columns).min(demos.len());

                                rect()
                                    .key(item.index)
                                    .height(Size::px(item.size))
                                    .horizontal()
                                    .spacing(GAP)
                                    .children(demos[start..end].iter().map(Demo::card))
                                    .into()
                            },
                        )
                        .length(demos.len().div_ceil(columns))
                        .item_size(CARD_HEIGHT + GAP)
                        .expanded(),
                    ),
            )
    }
}
