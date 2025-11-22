use freya_core::prelude::*;
use torin::prelude::*;

use crate::{
    get_theme,
    icons::tick::TickIcon,
    theming::component_themes::{
        CheckboxTheme,
        CheckboxThemePartial,
    },
};

/// Checkbox component.
///
/// # Example
///
/// ```rust
/// # use std::collections::HashSet;
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let mut checked = use_state(|| false);
///
///     rect()
///         .spacing(8.)
///         .child(
///             Tile::new()
///                 .on_select(move |_| checked.toggle())
///                 .child(Checkbox::new().selected(checked()))
///                 .leading("Click to check"),
///         )
///         .child(
///             Tile::new()
///                 .on_select(move |_| checked.toggle())
///                 .child(Checkbox::new().selected(!checked()))
///                 .child("Click to check"),
///         )
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect()
///         .spacing(8.).center().expanded().child(app())
/// # }, (250., 250.).into(), "./images/gallery_checkbox.png");
/// ```
///
/// # Preview
/// ![Checkbox Preview][checkbox]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("checkbox", "images/gallery_checkbox.png")
)]
#[derive(Clone, PartialEq)]
pub struct Checkbox {
    pub(crate) theme: Option<CheckboxThemePartial>,
    selected: bool,
    key: DiffKey,
    size: f32,
}

impl Default for Checkbox {
    fn default() -> Self {
        Self::new()
    }
}

impl Checkbox {
    pub fn new() -> Self {
        Self {
            selected: false,
            theme: None,
            key: DiffKey::None,
            size: 20.,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn theme(mut self, theme: CheckboxThemePartial) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn key(mut self, key: impl Into<DiffKey>) -> Self {
        self.key = key.into();
        self
    }

    pub fn size(mut self, size: impl Into<f32>) -> Self {
        self.size = size.into();
        self
    }
}

impl Render for Checkbox {
    fn render(&self) -> impl IntoElement {
        let focus = use_focus();
        let focus_status = use_focus_status(focus);
        let CheckboxTheme {
            border_fill,
            unselected_fill,
            selected_fill,
            selected_icon_fill,
        } = get_theme!(&self.theme, checkbox);

        let (background, fill) = if self.selected {
            (selected_fill, selected_fill)
        } else {
            (Color::TRANSPARENT, unselected_fill)
        };

        let border = Border::new()
            .fill(fill)
            .width(2.)
            .alignment(BorderAlignment::Inner);

        let focused_border = (focus_status() == FocusStatus::Keyboard).then(|| {
            Border::new()
                .fill(border_fill)
                .width((self.size * 0.15).ceil())
                .alignment(BorderAlignment::Outer)
        });

        rect()
            .a11y_id(focus.a11y_id())
            .a11y_focusable(Focusable::Enabled)
            .width(Size::px(self.size))
            .height(Size::px(self.size))
            .padding(Gaps::new_all(4.0))
            .main_align(Alignment::center())
            .cross_align(Alignment::center())
            .corner_radius(CornerRadius::new_all(self.size * 0.24))
            .border(border)
            .border(focused_border)
            .background(background)
            .on_key_down({
                move |e: Event<KeyboardEventData>| {
                    if !Focus::is_pressed(&e) {
                        e.stop_propagation();
                    }
                }
            })
            .maybe_child(self.selected.then(|| {
                TickIcon::new()
                    .width(Size::px(self.size * 0.7))
                    .height(Size::px(self.size * 0.7))
                    .fill(selected_icon_fill)
            }))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
