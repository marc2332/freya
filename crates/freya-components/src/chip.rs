use freya_core::prelude::*;
use torin::size::Size;

use crate::{
    get_theme,
    icons::tick::TickIcon,
    theming::component_themes::{
        ChipTheme,
        ChipThemePartial,
    },
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ChipStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the chip.
    Hovering,
}

// TODO: Add layout and style variants
// TODO: Ability to hide/customize icon
#[derive(PartialEq)]
pub struct Chip {
    pub theme: Option<ChipThemePartial>,
    pub children: Vec<Element>,
    pub on_press: Option<EventHandler<Event<PressEventData>>>,
    pub selected: bool,
    pub enabled: bool,
    key: DiffKey,
}

impl Default for Chip {
    fn default() -> Self {
        Self {
            theme: None,
            children: Vec::new(),
            on_press: None,
            selected: false,
            enabled: true,
            key: DiffKey::None,
        }
    }
}

impl Chip {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn theme(mut self, theme: ChipThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn selected(mut self, selected: impl Into<bool>) -> Self {
        self.selected = selected.into();
        self
    }

    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = enabled.into();
        self
    }

    pub fn on_press(mut self, handler: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_press = Some(handler.into());
        self
    }
}

impl ChildrenExt for Chip {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for Chip {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Render for Chip {
    fn render(&self) -> Element {
        let theme = get_theme!(&self.theme, chip);
        let mut status = use_state(|| ChipStatus::Idle);
        let focus = use_focus();
        let focus_status = use_focus_status(focus);

        let ChipTheme {
            background,
            hover_background,
            selected_background,
            border_fill,
            selected_border_fill,
            hover_border_fill,
            focus_border_fill,
            padding,
            margin,
            corner_radius,
            width,
            height,
            color,
            hover_color,
            selected_color,
            hover_icon_fill,
            selected_icon_fill,
        } = theme;

        let on_press = self.on_press.clone();
        let on_press = move |e: Event<PressEventData>| {
            focus.request_focus();
            if let Some(on_press) = &on_press {
                on_press.call(e);
            }
        };

        let on_pointer_enter = move |_| {
            Cursor::set(CursorIcon::Pointer);
            status.set(ChipStatus::Hovering);
        };

        let on_pointer_leave = move |_| {
            if status() == ChipStatus::Hovering {
                Cursor::set(CursorIcon::default());
                status.set(ChipStatus::Idle);
            }
        };

        let background = match status() {
            ChipStatus::Hovering => hover_background,
            _ if self.selected => selected_background,
            _ => background,
        };
        let color = match status() {
            ChipStatus::Hovering => hover_color,
            _ if self.selected => selected_color,
            _ => color,
        };
        let border_fill = match status() {
            ChipStatus::Hovering => hover_border_fill,
            _ if self.selected => selected_border_fill,
            _ => border_fill,
        };
        let icon_fill = match status() {
            ChipStatus::Hovering if self.selected => Some(hover_icon_fill),
            _ if self.selected => Some(selected_icon_fill),
            _ => None,
        };
        let border = if focus_status() == FocusStatus::Keyboard {
            Border::new()
                .fill(focus_border_fill)
                .width(2.)
                .alignment(BorderAlignment::Inner)
        } else {
            Border::new()
                .fill(border_fill.mul_if(!self.enabled, 0.9))
                .width(1.)
                .alignment(BorderAlignment::Inner)
        };

        rect()
            .a11y_id(focus.a11y_id())
            .a11y_focusable(self.enabled)
            .a11y_role(AccessibilityRole::Button)
            .maybe(self.enabled, |rect| {
                rect.on_press(on_press).on_pointer_enter(on_pointer_enter)
            })
            .on_pointer_leave(on_pointer_leave)
            .width(width)
            .height(height)
            .padding(padding)
            .margin(margin)
            .overflow_mode(OverflowMode::Clip)
            .border(border)
            .corner_radius(corner_radius)
            .color(color.mul_if(!self.enabled, 0.9))
            .background(background.mul_if(!self.enabled, 0.9))
            .center()
            .horizontal()
            .spacing(4.)
            .maybe_child(icon_fill.map(|icon_fill| {
                TickIcon::new()
                    .fill(icon_fill)
                    .width(Size::px(12.))
                    .height(Size::px(12.))
            }))
            .children(self.children.clone())
            .into()
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
