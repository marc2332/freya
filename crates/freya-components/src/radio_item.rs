use freya_animation::prelude::*;
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
/// fn app() -> impl IntoElement {
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
/// }
///
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().spacing(8.).center().expanded().child(app())
/// # }, "./images/gallery_radio.png").render();
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
    fn render(&self) -> impl IntoElement {
        let focus = use_focus();
        let focus_status = use_focus_status(focus);
        let RadioItemTheme {
            unselected_fill,
            selected_fill,
            border_fill,
        } = get_theme!(&self.theme, radio);

        let animation = use_animation_with_dependencies(&self.selected, move |conf, selected| {
            conf.on_change(OnChange::Rerun);
            conf.on_creation(OnCreation::Finish);

            let scale = AnimNum::new(0.7, 1.)
                .time(250)
                .ease(Ease::Out)
                .function(Function::Expo);
            let opacity = AnimNum::new(0., 1.)
                .time(250)
                .ease(Ease::Out)
                .function(Function::Expo);

            if *selected {
                (scale, opacity)
            } else {
                (scale.into_reversed(), opacity.into_reversed())
            }
        });

        let (scale, opacity) = animation.read().value();

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
            .a11y_role(AccessibilityRole::RadioButton)
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
            .maybe_child((self.selected || opacity > 0.).then(|| {
                rect()
                    .opacity(opacity)
                    .scale(scale)
                    .width(Size::px((self.size * 0.55).floor()))
                    .height(Size::px((self.size * 0.55).floor()))
                    .background(fill)
                    .corner_radius(CornerRadius::new_all(99.))
            }))
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

pub fn radio() -> RadioItem {
    RadioItem::new()
}
