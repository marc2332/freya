mod animation;
mod graphics;
mod i18n;
mod kanban;
mod markdown;
mod material;
mod scroll;
mod widgets;

use freya::prelude::*;

pub use crate::showcases::{
    animation::AnimationShowcase,
    graphics::GraphicsShowcase,
    i18n::I18nShowcase,
    kanban::KanbanShowcase,
    markdown::MarkdownShowcase,
    material::MaterialShowcase,
    scroll::ScrollShowcase,
    widgets::WidgetsShowcase,
};

/// Title and subtitle shown at the top of every showcase.
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
