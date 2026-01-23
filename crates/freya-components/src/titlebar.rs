use freya_core::prelude::*;
use torin::size::Size;

use crate::{
    get_theme,
    theming::component_themes::TitlebarButtonThemePartial,
};

#[derive(Clone, PartialEq, Copy)]
pub enum TitlebarAction {
    Minimize,
    Maximize,
    Close,
}

/// Titlebar button component.
#[derive(PartialEq)]
pub struct TitlebarButton {
    pub(crate) theme: Option<TitlebarButtonThemePartial>,
    pub(crate) action: TitlebarAction,
    pub(crate) on_press: Option<EventHandler<Event<PressEventData>>>,
    key: DiffKey,
}

impl KeyExt for TitlebarButton {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl TitlebarButton {
    pub fn new(action: TitlebarAction) -> Self {
        Self {
            theme: None,
            action,
            on_press: None,
            key: DiffKey::None,
        }
    }

    pub fn on_press(mut self, on_press: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_press = Some(on_press.into());
        self
    }
}

impl Component for TitlebarButton {
    fn render(&self) -> impl IntoElement {
        let mut hovering = use_state(|| false);
        let theme = get_theme!(&self.theme, titlebar_button);

        let icon_svg = match self.action {
            TitlebarAction::Minimize => {
                r#"<svg viewBox="0 0 12 12"><rect x="1" y="5" width="10" height="2" fill="currentColor"/></svg>"#
            }
            TitlebarAction::Maximize => {
                r#"<svg viewBox="0 0 12 12"><rect x="2" y="2" width="8" height="8" fill="none" stroke="currentColor" stroke-width="1.5"/></svg>"#
            }
            TitlebarAction::Close => {
                r#"<svg viewBox="0 0 12 12"><path d="M3 3l6 6M9 3l-6 6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>"#
            }
        };

        let icon = svg(Bytes::from_static(icon_svg.as_bytes()))
            .width(Size::px(12.))
            .height(Size::px(12.));

        let background = if hovering() {
            theme.hover_background
        } else {
            theme.background
        };

        rect()
            .width(theme.width)
            .height(theme.height)
            .background(background)
            .center()
            .on_pointer_enter(move |_| {
                hovering.set(true);
            })
            .on_pointer_leave(move |_| {
                hovering.set(false);
            })
            .map(self.on_press.clone(), |el, on_press| el.on_press(on_press))
            .child(icon)
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
