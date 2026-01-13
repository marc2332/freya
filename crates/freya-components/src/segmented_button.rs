use freya_core::prelude::*;
use torin::size::Size;

use crate::{
    get_theme,
    icons::tick::TickIcon,
    theming::component_themes::{
        ButtonSegmentTheme,
        ButtonSegmentThemePartial,
        SegmentedButtonTheme,
        SegmentedButtonThemePartial,
    },
};

/// Identifies the current status of the [`ButtonSegment`]s.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ButtonSegmentStatus {
    /// Default state.
    #[default]
    Idle,
    /// Pointer is hovering the button.
    Hovering,
}

/// A segment button to be used within a [`SegmentedButton`].
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// # use std::collections::HashSet;
/// fn app() -> impl IntoElement {
///     let mut selected = use_state(|| HashSet::from([1]));
///     SegmentedButton::new().children((0..2).map(|i| {
///         ButtonSegment::new()
///             .key(i)
///             .selected(selected.read().contains(&i))
///             .on_press(move |_| {
///                 if selected.read().contains(&i) {
///                     selected.write().remove(&i);
///                 } else {
///                     selected.write().insert(i);
///                 }
///             })
///             .child(format!("Option {i}"))
///             .into()
///     }))
/// }
/// ```
#[derive(Clone, PartialEq)]
pub struct ButtonSegment {
    pub(crate) theme: Option<ButtonSegmentThemePartial>,
    children: Vec<Element>,
    on_press: Option<EventHandler<Event<PressEventData>>>,
    selected: bool,
    enabled: bool,
    key: DiffKey,
}

impl Default for ButtonSegment {
    fn default() -> Self {
        Self::new()
    }
}

impl ButtonSegment {
    pub fn new() -> Self {
        Self {
            theme: None,
            children: Vec::new(),
            on_press: None,
            selected: false,
            enabled: true,
            key: DiffKey::None,
        }
    }

    pub fn selected(mut self, selected: impl Into<bool>) -> Self {
        self.selected = selected.into();
        self
    }

    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = enabled.into();
        self
    }

    pub fn on_press(mut self, on_press: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_press = Some(on_press.into());
        self
    }
}

impl ChildrenExt for ButtonSegment {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for ButtonSegment {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for ButtonSegment {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, button_segment);
        let mut status = use_state(|| ButtonSegmentStatus::Idle);
        let focus = use_focus();
        let focus_status = use_focus_status(focus);

        let ButtonSegmentTheme {
            background,
            hover_background,
            disabled_background,
            selected_background,
            focus_background,
            padding,
            selected_padding,
            width,
            height,
            color,
            selected_icon_fill,
        } = theme;

        let enabled = use_reactive(&self.enabled);
        use_drop(move || {
            if status() == ButtonSegmentStatus::Hovering && enabled() {
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
            status.set(ButtonSegmentStatus::Hovering);
            if enabled() {
                Cursor::set(CursorIcon::Pointer);
            } else {
                Cursor::set(CursorIcon::NotAllowed);
            }
        };

        let on_pointer_leave = move |_| {
            if status() == ButtonSegmentStatus::Hovering {
                Cursor::set(CursorIcon::default());
                status.set(ButtonSegmentStatus::Idle);
            }
        };

        let background = match status() {
            _ if !self.enabled => disabled_background,
            _ if self.selected => selected_background,
            ButtonSegmentStatus::Hovering => hover_background,
            ButtonSegmentStatus::Idle => background,
        };

        let padding = if self.selected {
            selected_padding
        } else {
            padding
        };
        let background = if *focus_status.read() == FocusStatus::Keyboard {
            focus_background
        } else {
            background
        };

        rect()
            .a11y_id(focus.a11y_id())
            .a11y_focusable(self.enabled)
            .a11y_role(AccessibilityRole::Button)
            .maybe(self.enabled, |rect| rect.on_press(on_press))
            .on_pointer_enter(on_pointer_enter)
            .on_pointer_leave(on_pointer_leave)
            .horizontal()
            .width(width)
            .height(height)
            .padding(padding)
            .overflow(Overflow::Clip)
            .color(color.mul_if(!self.enabled, 0.9))
            .background(background.mul_if(!self.enabled, 0.9))
            .center()
            .spacing(4.)
            .maybe_child(self.selected.then(|| {
                TickIcon::new()
                    .fill(selected_icon_fill)
                    .width(Size::px(12.))
                    .height(Size::px(12.))
            }))
            .children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// A container for grouping [`ButtonSegment`]s together.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// # use std::collections::HashSet;
/// fn app() -> impl IntoElement {
///     let mut selected = use_state(|| HashSet::from([1]));
///     SegmentedButton::new().children((0..2).map(|i| {
///         ButtonSegment::new()
///             .key(i)
///             .selected(selected.read().contains(&i))
///             .on_press(move |_| {
///                 if selected.read().contains(&i) {
///                     selected.write().remove(&i);
///                 } else {
///                     selected.write().insert(i);
///                 }
///             })
///             .child(format!("Option {i}"))
///             .into()
///     }))
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(app())
/// # }, "./images/gallery_segmented_button.png").render();
/// ```
///
/// # Preview
/// ![SegmentedButton Preview][segmented_button]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("segmented_button", "images/gallery_segmented_button.png")
)]
#[derive(Clone, PartialEq)]
pub struct SegmentedButton {
    pub(crate) theme: Option<SegmentedButtonThemePartial>,
    children: Vec<Element>,
    key: DiffKey,
}

impl Default for SegmentedButton {
    fn default() -> Self {
        Self::new()
    }
}

impl SegmentedButton {
    pub fn new() -> Self {
        Self {
            theme: None,
            children: Vec::new(),
            key: DiffKey::None,
        }
    }

    pub fn theme(mut self, theme: SegmentedButtonThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }
}

impl ChildrenExt for SegmentedButton {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl KeyExt for SegmentedButton {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for SegmentedButton {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, segmented_button);

        let SegmentedButtonTheme {
            background,
            border_fill,
            corner_radius,
        } = theme;

        rect()
            .overflow(Overflow::Clip)
            .background(background)
            .border(
                Border::new()
                    .fill(border_fill)
                    .width(1.)
                    .alignment(BorderAlignment::Outer),
            )
            .corner_radius(corner_radius)
            .horizontal()
            .children(self.children.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
