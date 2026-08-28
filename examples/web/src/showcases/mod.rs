mod animation;
mod components;
mod graphics;
mod i18n;
mod kanban;
mod markdown;
mod material;
mod scroll;

use freya::prelude::*;

pub use crate::showcases::{
    animation::AnimationShowcase,
    components::ComponentsShowcase,
    graphics::GraphicsShowcase,
    i18n::I18nShowcase,
    kanban::KanbanShowcase,
    markdown::MarkdownShowcase,
    material::MaterialShowcase,
    scroll::ScrollShowcase,
};

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
