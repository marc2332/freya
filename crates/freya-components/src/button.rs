use freya_core::prelude::*;

use crate::{
    get_theme,
    theming::component_themes::{
        ButtonColorsThemePartial,
        ButtonLayoutThemePartial,
        ButtonLayoutThemePartialExt,
    },
};

#[derive(Clone, PartialEq)]
pub enum ButtonStyleVariant {
    Normal,
    Filled,
    Outline,
    Flat,
}

#[derive(Clone, PartialEq)]
pub enum ButtonLayoutVariant {
    Normal,
    Compact,
    Expanded,
}

/// Simply a button.
///
/// ## **Normal**
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     Button::new()
///         .on_press(|_| println!("Pressed!"))
///         .child("Press me")
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(app())
/// # }, (250., 250.).into(), "./images/gallery_button.png");
/// ```
/// ## **Filled**
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     Button::new()
///         .on_press(|_| println!("Pressed!"))
///         .filled()
///         .child("Press me")
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(app())
/// # }, (250., 250.).into(), "./images/gallery_filled_button.png");
/// ```
/// ## **Outline**
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     Button::new()
///         .on_press(|_| println!("Pressed!"))
///         .outline()
///         .child("Press me")
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(app())
/// # }, (250., 250.).into(), "./images/gallery_outline_button.png");
/// ```
/// ## **Flat**
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     Button::new()
///         .on_press(|_| println!("Pressed!"))
///         .flat()
///         .child("Press me")
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(app())
/// # }, (250., 250.).into(), "./images/gallery_flat_button.png");
/// ```
///
/// # Preview
/// ![Button Preview][button]
/// ![Outline Button Preview][outline_button]
/// ![Filled Button Preview][filled_button]
/// ![Flat Button Preview][flat_button]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("button", "images/gallery_button.png"),
    doc = embed_doc_image::embed_image!("filled_button", "images/gallery_filled_button.png"),
    doc = embed_doc_image::embed_image!("outline_button", "images/gallery_outline_button.png"),
    doc = embed_doc_image::embed_image!("flat_button", "images/gallery_flat_button.png"),
)]
#[derive(Clone, PartialEq)]
pub struct Button {
    pub(crate) theme_colors: Option<ButtonColorsThemePartial>,
    pub(crate) theme_layout: Option<ButtonLayoutThemePartial>,
    elements: Vec<Element>,
    on_press: Option<EventHandler<Event<PressEventData>>>,
    on_secondary_press: Option<EventHandler<Event<PressEventData>>>,
    key: DiffKey,
    style_variant: ButtonStyleVariant,
    layout_variant: ButtonLayoutVariant,
    enabled: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for Button {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl KeyExt for Button {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Button {
    pub fn new() -> Self {
        Self {
            theme_colors: None,
            theme_layout: None,
            style_variant: ButtonStyleVariant::Normal,
            layout_variant: ButtonLayoutVariant::Normal,
            on_press: None,
            on_secondary_press: None,
            elements: Vec::default(),
            enabled: true,
            key: DiffKey::None,
        }
    }

    pub fn get_layout_variant(&self) -> &ButtonLayoutVariant {
        &self.layout_variant
    }

    pub fn get_theme_layout(&self) -> Option<&ButtonLayoutThemePartial> {
        self.theme_layout.as_ref()
    }

    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = enabled.into();
        self
    }

    pub fn style_variant(mut self, style_variant: impl Into<ButtonStyleVariant>) -> Self {
        self.style_variant = style_variant.into();
        self
    }

    pub fn layout_variant(mut self, layout_variant: impl Into<ButtonLayoutVariant>) -> Self {
        self.layout_variant = layout_variant.into();
        self
    }

    pub fn on_press(mut self, on_press: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_press = Some(on_press.into());
        self
    }

    pub fn on_secondary_press(
        mut self,
        on_secondary_press: impl Into<EventHandler<Event<PressEventData>>>,
    ) -> Self {
        self.on_secondary_press = Some(on_secondary_press.into());
        self
    }

    pub fn theme_colors(mut self, theme: ButtonColorsThemePartial) -> Self {
        self.theme_colors = Some(theme);
        self
    }

    pub fn theme_layout(mut self, theme: ButtonLayoutThemePartial) -> Self {
        self.theme_layout = Some(theme);
        self
    }

    /// Shortcut for [Self::theme_layout] and [ButtonLayoutVariant::Compact].
    pub fn compact(self) -> Self {
        self.layout_variant(ButtonLayoutVariant::Compact)
    }

    /// Shortcut for [Self::theme_layout] and [ButtonLayoutVariant::Expanded].
    pub fn expanded(self) -> Self {
        self.layout_variant(ButtonLayoutVariant::Expanded)
    }

    /// Shortcut for [Self::style_variant] and [ButtonStyleVariant::Filled].
    pub fn filled(self) -> Self {
        self.style_variant(ButtonStyleVariant::Filled)
    }

    /// Shortcut for [Self::style_variant] and [ButtonStyleVariant::Outline].
    pub fn outline(self) -> Self {
        self.style_variant(ButtonStyleVariant::Outline)
    }

    /// Shortcut for [Self::style_variant] and [ButtonStyleVariant::Flat].
    pub fn flat(self) -> Self {
        self.style_variant(ButtonStyleVariant::Flat)
    }

    /// Shortcut for [Self::corner_radius] with `99`.
    pub fn rounded(self) -> Self {
        self.corner_radius(99.)
    }
}

impl Render for Button {
    fn render(&self) -> impl IntoElement {
        let mut hovering = use_state(|| false);
        let focus = use_focus();
        let focus_status = use_focus_status(focus);

        let enabled = use_reactive(&self.enabled);
        use_drop(move || {
            if hovering() && enabled() {
                Cursor::set(CursorIcon::default());
            }
        });

        let theme_colors = match self.style_variant {
            ButtonStyleVariant::Normal => get_theme!(&self.theme_colors, button),
            ButtonStyleVariant::Outline => get_theme!(&self.theme_colors, outline_button),
            ButtonStyleVariant::Filled => get_theme!(&self.theme_colors, filled_button),
            ButtonStyleVariant::Flat => get_theme!(&self.theme_colors, flat_button),
        };
        let theme_layout = match self.layout_variant {
            ButtonLayoutVariant::Normal => get_theme!(&self.theme_layout, button_layout),
            ButtonLayoutVariant::Compact => get_theme!(&self.theme_layout, compact_button_layout),
            ButtonLayoutVariant::Expanded => get_theme!(&self.theme_layout, expanded_button_layout),
        };

        let border = if focus_status() == FocusStatus::Keyboard {
            Border::new()
                .fill(theme_colors.focus_border_fill)
                .width(2.)
                .alignment(BorderAlignment::Inner)
        } else {
            Border::new()
                .fill(theme_colors.border_fill.mul_if(!self.enabled, 0.9))
                .width(1.)
                .alignment(BorderAlignment::Inner)
        };
        let background = if enabled() && hovering() {
            theme_colors.hover_background
        } else {
            theme_colors.background
        };

        rect()
            .overflow(Overflow::Clip)
            .a11y_id(focus.a11y_id())
            .a11y_focusable(self.enabled)
            .a11y_role(AccessibilityRole::Button)
            .background(background.mul_if(!self.enabled, 0.9))
            .border(border)
            .padding(theme_layout.padding)
            .corner_radius(theme_layout.corner_radius)
            .width(theme_layout.width)
            .height(theme_layout.height)
            .color(theme_colors.color.mul_if(!self.enabled, 0.9))
            .center()
            .maybe(self.enabled, |rect| {
                rect.on_all_press({
                    let on_press = self.on_press.clone();
                    let on_secondary_press = self.on_secondary_press.clone();
                    move |e: Event<PressEventData>| {
                        focus.request_focus();
                        match e.data() {
                            PressEventData::Mouse(data) => match data.button {
                                Some(MouseButton::Left) => {
                                    if let Some(handler) = &on_press {
                                        handler.call(e);
                                    }
                                }
                                Some(MouseButton::Right) => {
                                    if let Some(handler) = &on_secondary_press {
                                        handler.call(e);
                                    }
                                }
                                _ => {}
                            },
                            PressEventData::Touch(_) | PressEventData::Keyboard(_) => {
                                if let Some(handler) = &on_press {
                                    handler.call(e);
                                }
                            }
                        }
                    }
                })
            })
            .on_pointer_enter(move |_| {
                hovering.set(true);
                if enabled() {
                    Cursor::set(CursorIcon::Pointer);
                } else {
                    Cursor::set(CursorIcon::NotAllowed);
                }
            })
            .on_pointer_leave(move |_| {
                if hovering() {
                    Cursor::set(CursorIcon::default());
                    hovering.set(false);
                }
            })
            .children(self.elements.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
