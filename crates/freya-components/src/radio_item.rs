use freya_core::prelude::*;
use torin::prelude::*;

use crate::{
    get_theme,
    theming::component_themes::{
        RadioItemTheme,
        RadioItemThemePartial,
    },
};

/// Radio component.
///
/// # Example
///
/// ```rust
/// # use std::collections::HashSet;
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let mut checked = use_state(|| false);
///
///     rect()
///         .child(
///             Tile::new()
///                 .on_select(move |_| checked.toggle())
///                 .child(RadioItem::new().selected(checked()))
///                 .leading("Click to check"),
///         )
///         .child(
///             Tile::new()
///                 .on_select(move |_| checked.toggle())
///                 .child(RadioItem::new().selected(!checked()))
///                 .child("Click to check"),
///         )
///         .into()
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect()
///         .spacing(8.).center().expanded().child(app()).into()
/// # }, (250., 250.).into(), "./images/gallery_radio.png");
/// ```
///
/// # Preview
/// ![Radio Preview][radio]
#[cfg_attr(feature = "docs",
    doc = embed_doc_image::embed_image!("radio", "images/gallery_radio.png")
)]
#[derive(Clone, PartialEq)]
pub struct RadioItem {
    pub(crate) theme: Option<RadioItemThemePartial>,
    key: DiffKey,
    selected: bool,
    size: f32,
}

impl Default for RadioItem {
    fn default() -> Self {
        Self::new()
    }
}

impl RadioItem {
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

    pub fn theme(mut self, theme: RadioItemThemePartial) -> Self {
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

impl Render for RadioItem {
    fn render(&self) -> Element {
        let focus = use_focus();
        let focus_status = use_focus_status(focus);
        let RadioItemTheme {
            unselected_fill,
            selected_fill,
            border_fill,
        } = get_theme!(&self.theme, radio);

        let fill = if self.selected {
            selected_fill
        } else {
            unselected_fill
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
            .border(border)
            .border(focused_border)
            .padding(Gaps::new_all(4.0))
            .main_align(Alignment::center())
            .cross_align(Alignment::center())
            .corner_radius(CornerRadius::new_all(99.))
            .on_key_down(move |e: Event<KeyboardEventData>| {
                if !Focus::is_pressed(&e) {
                    e.stop_propagation();
                }
            })
            .maybe_child(self.selected.then(|| {
                rect()
                    .width(Size::px((self.size * 0.55).floor()))
                    .height(Size::px((self.size * 0.55).floor()))
                    .background(fill)
                    .corner_radius(CornerRadius::new_all(99.))
            }))
            .into()
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

pub fn radio() -> RadioItem {
    RadioItem::new()
}
