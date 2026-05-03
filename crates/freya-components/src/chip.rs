use freya_core::prelude::*;
use torin::{
    gaps::Gaps,
    size::Size,
};

use crate::{
    define_theme,
    get_theme,
    icons::tick::TickIcon,
};

define_theme! {
    %[component]
    pub Chip {
        %[fields]
        background: Color,
        hover_background: Color,
        selected_background: Color,
        border_fill: Color,
        selected_border_fill: Color,
        hover_border_fill: Color,
        focus_border_fill: Color,
        margin: f32,
        corner_radius: CornerRadius,
        width: Size,
        height: Size,
        padding: Gaps,
        color: Color,
        hover_color: Color,
        selected_color: Color,
        selected_icon_fill: Color,
        hover_icon_fill: Color,
    }
}

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
///
/// Chip component.
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     Chip::new().child("Chip")
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(app())
/// # }, "./images/gallery_chip.png").render();
/// ```
///
/// # Preview
/// ![Chip Preview][chip]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("chip", "images/gallery_chip.png"),
)]
#[derive(Clone, PartialEq)]
pub struct Chip {
    pub(crate) theme: Option<ChipThemePartial>,
    children: Vec<Element>,
    on_press: Option<EventHandler<Event<PressEventData>>>,
    selected: bool,
    enabled: bool,
    cursor_icon: CursorIcon,
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
            cursor_icon: CursorIcon::Pointer,
            key: DiffKey::None,
        }
    }
}

impl Chip {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the theme override for this component.
    pub fn get_theme(&self) -> Option<&ChipThemePartial> {
        self.theme.as_ref()
    }

    /// Set a theme override for this component.
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

    /// Override the cursor icon shown when hovering over this component while enabled.
    pub fn cursor_icon(mut self, cursor_icon: impl Into<CursorIcon>) -> Self {
        self.cursor_icon = cursor_icon.into();
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

impl Component for Chip {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, ChipThemePreference, "chip");
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

        let enabled = use_reactive(&self.enabled);
        let cursor_icon = self.cursor_icon;
        use_drop(move || {
            if status() == ChipStatus::Hovering && enabled() {
                Cursor::set(CursorIcon::default());
            }
        });

        let on_press = self.on_press.clone();
        let on_press = move |e: Event<PressEventData>| {
            focus.request_focus();
            if let Some(on_press) = &on_press {
                on_press.call(e);
            }
        };

        let on_pointer_enter = move |_| {
            status.set(ChipStatus::Hovering);
            if enabled() {
                Cursor::set(cursor_icon);
            } else {
                Cursor::set(CursorIcon::NotAllowed);
            }
        };

        let on_pointer_leave = move |_| {
            if status() == ChipStatus::Hovering {
                Cursor::set(CursorIcon::default());
                status.set(ChipStatus::Idle);
            }
        };

        let background = match status() {
            ChipStatus::Hovering if enabled() => hover_background,
            _ if self.selected => selected_background,
            _ => background,
        };
        let color = match status() {
            ChipStatus::Hovering if enabled() => hover_color,
            _ if self.selected => selected_color,
            _ => color,
        };
        let border_fill = match status() {
            ChipStatus::Hovering if enabled() => hover_border_fill,
            _ if self.selected => selected_border_fill,
            _ => border_fill,
        };
        let icon_fill = match status() {
            ChipStatus::Hovering if self.selected && enabled() => Some(hover_icon_fill),
            _ if self.selected => Some(selected_icon_fill),
            _ => None,
        };
        let border = if self.enabled && focus_status() == FocusStatus::Keyboard {
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
            .maybe(self.enabled, |rect| rect.on_press(on_press))
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .width(width)
            .height(height)
            .padding(padding)
            .margin(margin)
            .overflow(Overflow::Clip)
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
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
