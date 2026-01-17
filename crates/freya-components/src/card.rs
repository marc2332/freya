use freya_core::prelude::*;

use crate::{
    get_theme,
    theming::component_themes::{
        CardColorsThemePartial, CardLayoutThemePartial, CardLayoutThemePartialExt,
    },
};

/// Style variants for the Card component.
#[derive(Clone, PartialEq)]
pub enum CardStyleVariant {
    Filled,
    Outline,
}

/// Layout variants for the Card component.
#[derive(Clone, PartialEq)]
pub enum CardLayoutVariant {
    Normal,
    Compact,
}

/// A container component with styling variants.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     Card::new()
///         .width(Size::percent(75.))
///         .height(Size::percent(75.))
///         .child("Hello, World!")
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(app())
/// # }, "./images/gallery_card.png").render();
/// ```
///
/// # Preview
/// ![Card Preview][card]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("card", "images/gallery_card.png"),
)]
#[derive(Clone, PartialEq)]
pub struct Card {
    pub(crate) theme_colors: Option<CardColorsThemePartial>,
    pub(crate) theme_layout: Option<CardLayoutThemePartial>,
    layout: LayoutData,
    accessibility: AccessibilityData,
    elements: Vec<Element>,
    on_press: Option<EventHandler<Event<PressEventData>>>,
    key: DiffKey,
    style_variant: CardStyleVariant,
    layout_variant: CardLayoutVariant,
    hoverable: bool,
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl ChildrenExt for Card {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl KeyExt for Card {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for Card {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerExt for Card {}

impl AccessibilityExt for Card {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.accessibility
    }
}

impl CornerRadiusExt for Card {
    fn with_corner_radius(self, corner_radius: f32) -> Self {
        self.corner_radius(corner_radius)
    }
}

impl Card {
    pub fn new() -> Self {
        Self {
            theme_colors: None,
            theme_layout: None,
            layout: LayoutData::default(),
            accessibility: AccessibilityData::default(),
            style_variant: CardStyleVariant::Outline,
            layout_variant: CardLayoutVariant::Normal,
            on_press: None,
            elements: Vec::default(),
            hoverable: false,
            key: DiffKey::None,
        }
    }

    /// Set the style variant.
    pub fn style_variant(mut self, style_variant: impl Into<CardStyleVariant>) -> Self {
        self.style_variant = style_variant.into();
        self
    }

    /// Set the layout variant.
    pub fn layout_variant(mut self, layout_variant: impl Into<CardLayoutVariant>) -> Self {
        self.layout_variant = layout_variant.into();
        self
    }

    /// Set whether the card should respond to hover interactions.
    pub fn hoverable(mut self, hoverable: impl Into<bool>) -> Self {
        self.hoverable = hoverable.into();
        self
    }

    /// Set the press event handler.
    pub fn on_press(mut self, on_press: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_press = Some(on_press.into());
        self
    }

    /// Set custom color theme.
    pub fn theme_colors(mut self, theme: CardColorsThemePartial) -> Self {
        self.theme_colors = Some(theme);
        self
    }

    /// Set custom layout theme.
    pub fn theme_layout(mut self, theme: CardLayoutThemePartial) -> Self {
        self.theme_layout = Some(theme);
        self
    }

    /// Shortcut for [Self::style_variant] with [CardStyleVariant::Filled].
    pub fn filled(self) -> Self {
        self.style_variant(CardStyleVariant::Filled)
    }

    /// Shortcut for [Self::style_variant] with [CardStyleVariant::Outline].
    pub fn outline(self) -> Self {
        self.style_variant(CardStyleVariant::Outline)
    }

    /// Shortcut for [Self::layout_variant] with [CardLayoutVariant::Compact].
    pub fn compact(self) -> Self {
        self.layout_variant(CardLayoutVariant::Compact)
    }
}

impl Component for Card {
    fn render(&self) -> impl IntoElement {
        let mut hovering = use_state(|| false);
        let focus = use_focus();
        let focus_status = use_focus_status(focus);

        let is_interactive = self.hoverable || self.on_press.is_some();

        use_drop(move || {
            if hovering() && is_interactive {
                Cursor::set(CursorIcon::default());
            }
        });

        let theme_colors = match self.style_variant {
            CardStyleVariant::Filled => get_theme!(&self.theme_colors, filled_card),
            CardStyleVariant::Outline => get_theme!(&self.theme_colors, outline_card),
        };
        let theme_layout = match self.layout_variant {
            CardLayoutVariant::Normal => get_theme!(&self.theme_layout, card_layout),
            CardLayoutVariant::Compact => get_theme!(&self.theme_layout, compact_card_layout),
        };

        let border = if focus_status() == FocusStatus::Keyboard {
            Border::new()
                .fill(theme_colors.border_fill)
                .width(2.)
                .alignment(BorderAlignment::Inner)
        } else {
            Border::new()
                .fill(theme_colors.border_fill)
                .width(1.)
                .alignment(BorderAlignment::Inner)
        };

        let background = if is_interactive && hovering() {
            theme_colors.hover_background
        } else {
            theme_colors.background
        };

        let shadow = if is_interactive && hovering() {
            Some(Shadow::new().y(4.).blur(8.).color(theme_colors.shadow))
        } else {
            None
        };

        rect()
            .layout(self.layout.clone())
            .overflow(Overflow::Clip)
            .a11y_id(focus.a11y_id())
            .a11y_focusable(is_interactive)
            .a11y_role(AccessibilityRole::GenericContainer)
            .accessibility(self.accessibility.clone())
            .background(background)
            .border(border)
            .padding(theme_layout.padding)
            .corner_radius(theme_layout.corner_radius)
            .color(theme_colors.color)
            .map(shadow, |rect, shadow| rect.shadow(shadow))
            .maybe(self.on_press.is_some(), |rect| {
                rect.on_press({
                    let on_press = self.on_press.clone();
                    move |e: Event<PressEventData>| {
                        focus.request_focus();
                        if let Some(handler) = &on_press {
                            handler.call(e);
                        }
                    }
                })
            })
            .maybe(is_interactive, |rect| {
                rect.on_pointer_enter(move |_| {
                    hovering.set(true);
                    Cursor::set(CursorIcon::Pointer);
                })
                .on_pointer_leave(move |_| {
                    if hovering() {
                        Cursor::set(CursorIcon::default());
                        hovering.set(false);
                    }
                })
            })
            .children(self.elements.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
