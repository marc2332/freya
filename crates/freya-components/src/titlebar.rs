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
        let theme = get_theme!(&self.theme, titlebar_button);
        let action = self.action.clone();

        let icon = match &action {
            TitlebarAction::Minimize => svg(freya_icons::lucide::minus())
                .width(Size::px(12.))
                .height(Size::px(12.)),
            TitlebarAction::Maximize => svg(freya_icons::lucide::maximize())
                .width(Size::px(12.))
                .height(Size::px(12.)),
            TitlebarAction::Close => svg(freya_icons::lucide::x())
                .width(Size::px(12.))
                .height(Size::px(12.)),
        };

        rect()
            .width(theme.width)
            .height(theme.height)
            .background(theme.background)
            .center()
            .map(self.on_press.clone(), |el, on_press| el.on_press(on_press))
            .child(icon)
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
