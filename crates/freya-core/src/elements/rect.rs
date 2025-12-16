use std::{
    any::Any,
    borrow::Cow,
    rc::Rc,
};

use freya_engine::prelude::{
    Canvas,
    ClipOp,
    Paint,
    PaintStyle,
    SkBlurStyle,
    SkMaskFilter,
    SkPath,
    SkPathFillType,
    SkPoint,
    SkRRect,
    SkRect,
};
use rustc_hash::FxHashMap;
use torin::{
    prelude::Area,
    scaled::Scaled,
};

use crate::{
    diff_key::DiffKey,
    element::{
        ClipContext,
        ElementExt,
        EventHandlerType,
        EventMeasurementContext,
        RenderContext,
    },
    events::name::EventName,
    layers::Layer,
    prelude::*,
    style::{
        fill::Fill,
        font_size::FontSize,
        gradient::{
            ConicGradient,
            LinearGradient,
            RadialGradient,
        },
        scale::Scale,
        shadow::{
            Shadow,
            ShadowPosition,
        },
    },
    tree::DiffModifies,
};

/// [rect] acts as a generic container to contain other elements inside, like a box.
///
/// Its the equivalent of `view`/`div`/`container` in other UI models.
///
/// See the available methods in [Rect].
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     rect().expanded().background((0, 255, 0))
/// }
/// ```
pub fn rect() -> Rect {
    Rect::empty()
}

#[derive(PartialEq, Clone)]
pub struct RectElement {
    pub style: StyleState,
    pub layout: LayoutData,
    pub text_style_data: TextStyleData,
    pub relative_layer: Layer,
    pub event_handlers: FxHashMap<EventName, EventHandlerType>,
    pub accessibility: AccessibilityData,
    pub effect: Option<EffectData>,
}

impl Default for RectElement {
    fn default() -> Self {
        let mut accessibility = AccessibilityData::default();
        accessibility
            .builder
            .set_role(accesskit::Role::GenericContainer);
        Self {
            style: Default::default(),
            layout: Default::default(),
            text_style_data: Default::default(),
            relative_layer: Default::default(),
            event_handlers: Default::default(),
            accessibility,
            effect: Default::default(),
        }
    }
}

impl RectElement {
    pub fn container_rect(&self, area: &Area, scale_factor: f32) -> SkRRect {
        let style = self.style();
        let corner_radius = style.corner_radius.with_scale(scale_factor);
        SkRRect::new_rect_radii(
            SkRect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
            &[
                (corner_radius.top_left, corner_radius.top_left).into(),
                (corner_radius.top_right, corner_radius.top_right).into(),
                (corner_radius.bottom_right, corner_radius.bottom_right).into(),
                (corner_radius.bottom_left, corner_radius.bottom_left).into(),
            ],
        )
    }

    pub fn render_shadow(
        canvas: &Canvas,
        path: &mut SkPath,
        rounded_rect: SkRRect,
        area: Area,
        shadow: &Shadow,
        corner_radius: &CornerRadius,
    ) {
        let mut shadow_path = SkPath::new();
        let mut shadow_paint = Paint::default();
        shadow_paint.set_anti_alias(true);
        shadow_paint.set_color(shadow.color);

        // Shadows can be either outset or inset
        // If they are outset, we fill a copy of the path outset by spread_radius, and blur it.
        // Otherwise, we draw a stroke with the inner portion being spread_radius width, and the outer portion being blur_radius width.
        let outset: SkPoint = match shadow.position {
            ShadowPosition::Normal => {
                shadow_paint.set_style(PaintStyle::Fill);
                (shadow.spread, shadow.spread).into()
            }
            ShadowPosition::Inset => {
                shadow_paint.set_style(PaintStyle::Stroke);
                shadow_paint.set_stroke_width(shadow.blur / 2.0 + shadow.spread);
                (-shadow.spread / 2.0, -shadow.spread / 2.0).into()
            }
        };

        // Apply gassuan blur to the copied path.
        if shadow.blur > 0.0 {
            shadow_paint.set_mask_filter(SkMaskFilter::blur(
                SkBlurStyle::Normal,
                shadow.blur / 2.0,
                false,
            ));
        }

        // Add either the RRect or smoothed path based on whether smoothing is used.
        if corner_radius.smoothing > 0.0 {
            shadow_path.add_path(
                &corner_radius.smoothed_path(rounded_rect.with_outset(outset)),
                SkPoint::new(area.min_x(), area.min_y()) - outset,
                None,
            );
        } else {
            shadow_path.add_rrect(rounded_rect.with_outset(outset), None);
        }

        // Offset our path by the shadow's x and y coordinates.
        shadow_path.offset((shadow.x, shadow.y));

        // Exclude the original path bounds from the shadow using a clip, then draw the shadow.
        canvas.save();
        canvas.clip_path(
            path,
            match shadow.position {
                ShadowPosition::Normal => ClipOp::Difference,
                ShadowPosition::Inset => ClipOp::Intersect,
            },
            true,
        );
        canvas.draw_path(&shadow_path, &shadow_paint);
        canvas.restore();
    }

    pub fn render_border(
        canvas: &Canvas,
        rect: SkRect,
        border: &Border,
        corner_radius: &CornerRadius,
    ) {
        let mut border_paint = Paint::default();
        border_paint.set_style(PaintStyle::Fill);
        border_paint.set_anti_alias(true);
        border_paint.set_color(border.fill);

        match Self::border_shape(rect, corner_radius, border) {
            BorderShape::DRRect(outer, inner) => {
                canvas.draw_drrect(outer, inner, &border_paint);
            }
            BorderShape::Path(path) => {
                canvas.draw_path(&path, &border_paint);
            }
        }
    }

    /// Returns a `Path` that will draw a [`Border`] around a base rectangle.
    ///
    /// We don't use Skia's stroking API here, since we might need different widths for each side.
    pub fn border_shape(
        base_rect: SkRect,
        base_corner_radius: &CornerRadius,
        border: &Border,
    ) -> BorderShape {
        let border_alignment = border.alignment;
        let border_width = border.width;

        // First we create a path that is outset from the rect by a certain amount on each side.
        //
        // Let's call this the outer border path.
        let (outer_rrect, outer_corner_radius) = {
            // Calculuate the outer corner radius for the border.
            let corner_radius = CornerRadius {
                top_left: Self::outer_border_path_corner_radius(
                    border_alignment,
                    base_corner_radius.top_left,
                    border_width.top,
                    border_width.left,
                ),
                top_right: Self::outer_border_path_corner_radius(
                    border_alignment,
                    base_corner_radius.top_right,
                    border_width.top,
                    border_width.right,
                ),
                bottom_left: Self::outer_border_path_corner_radius(
                    border_alignment,
                    base_corner_radius.bottom_left,
                    border_width.bottom,
                    border_width.left,
                ),
                bottom_right: Self::outer_border_path_corner_radius(
                    border_alignment,
                    base_corner_radius.bottom_right,
                    border_width.bottom,
                    border_width.right,
                ),
                smoothing: base_corner_radius.smoothing,
            };

            let rrect = SkRRect::new_rect_radii(
                {
                    let mut rect = base_rect;
                    let alignment_scale = match border_alignment {
                        BorderAlignment::Outer => 1.0,
                        BorderAlignment::Center => 0.5,
                        BorderAlignment::Inner => 0.0,
                    };

                    rect.left -= border_width.left * alignment_scale;
                    rect.top -= border_width.top * alignment_scale;
                    rect.right += border_width.right * alignment_scale;
                    rect.bottom += border_width.bottom * alignment_scale;

                    rect
                },
                &[
                    (corner_radius.top_left, corner_radius.top_left).into(),
                    (corner_radius.top_right, corner_radius.top_right).into(),
                    (corner_radius.bottom_right, corner_radius.bottom_right).into(),
                    (corner_radius.bottom_left, corner_radius.bottom_left).into(),
                ],
            );

            (rrect, corner_radius)
        };

        // After the outer path, we will then move to the inner bounds of the border.
        let (inner_rrect, inner_corner_radius) = {
            // Calculuate the inner corner radius for the border.
            let corner_radius = CornerRadius {
                top_left: Self::inner_border_path_corner_radius(
                    border_alignment,
                    base_corner_radius.top_left,
                    border_width.top,
                    border_width.left,
                ),
                top_right: Self::inner_border_path_corner_radius(
                    border_alignment,
                    base_corner_radius.top_right,
                    border_width.top,
                    border_width.right,
                ),
                bottom_left: Self::inner_border_path_corner_radius(
                    border_alignment,
                    base_corner_radius.bottom_left,
                    border_width.bottom,
                    border_width.left,
                ),
                bottom_right: Self::inner_border_path_corner_radius(
                    border_alignment,
                    base_corner_radius.bottom_right,
                    border_width.bottom,
                    border_width.right,
                ),
                smoothing: base_corner_radius.smoothing,
            };

            let rrect = SkRRect::new_rect_radii(
                {
                    let mut rect = base_rect;
                    let alignment_scale = match border_alignment {
                        BorderAlignment::Outer => 0.0,
                        BorderAlignment::Center => 0.5,
                        BorderAlignment::Inner => 1.0,
                    };

                    rect.left += border_width.left * alignment_scale;
                    rect.top += border_width.top * alignment_scale;
                    rect.right -= border_width.right * alignment_scale;
                    rect.bottom -= border_width.bottom * alignment_scale;

                    rect
                },
                &[
                    (corner_radius.top_left, corner_radius.top_left).into(),
                    (corner_radius.top_right, corner_radius.top_right).into(),
                    (corner_radius.bottom_right, corner_radius.bottom_right).into(),
                    (corner_radius.bottom_left, corner_radius.bottom_left).into(),
                ],
            );

            (rrect, corner_radius)
        };

        if base_corner_radius.smoothing > 0.0 {
            let mut path = SkPath::new();
            path.set_fill_type(SkPathFillType::EvenOdd);

            path.add_path(
                &outer_corner_radius.smoothed_path(outer_rrect),
                SkPoint::new(outer_rrect.rect().x(), outer_rrect.rect().y()),
                None,
            );

            path.add_path(
                &inner_corner_radius.smoothed_path(inner_rrect),
                SkPoint::new(inner_rrect.rect().x(), inner_rrect.rect().y()),
                None,
            );

            BorderShape::Path(path)
        } else {
            BorderShape::DRRect(outer_rrect, inner_rrect)
        }
    }

    fn outer_border_path_corner_radius(
        alignment: BorderAlignment,
        corner_radius: f32,
        width_1: f32,
        width_2: f32,
    ) -> f32 {
        if alignment == BorderAlignment::Inner || corner_radius == 0.0 {
            return corner_radius;
        }

        let mut offset = if width_1 == 0.0 {
            width_2
        } else if width_2 == 0.0 {
            width_1
        } else {
            width_1.min(width_2)
        };

        if alignment == BorderAlignment::Center {
            offset *= 0.5;
        }

        corner_radius + offset
    }

    fn inner_border_path_corner_radius(
        alignment: BorderAlignment,
        corner_radius: f32,
        width_1: f32,
        width_2: f32,
    ) -> f32 {
        if alignment == BorderAlignment::Outer || corner_radius == 0.0 {
            return corner_radius;
        }

        let mut offset = if width_1 == 0.0 {
            width_2
        } else if width_2 == 0.0 {
            width_1
        } else {
            width_1.min(width_2)
        };

        if alignment == BorderAlignment::Center {
            offset *= 0.5;
        }

        corner_radius - offset
    }
}

impl ElementExt for RectElement {
    fn changed(&self, other: &Rc<dyn ElementExt>) -> bool {
        let Some(rect) = (other.as_ref() as &dyn Any).downcast_ref::<Self>() else {
            return false;
        };

        self != rect
    }

    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(rect) = (other.as_ref() as &dyn Any).downcast_ref::<Self>() else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if self.style != rect.style {
            diff.insert(DiffModifies::STYLE);
        }

        if self.effect != rect.effect {
            diff.insert(DiffModifies::EFFECT);
        }

        if !self.layout.layout.self_layout_eq(&rect.layout.layout) {
            diff.insert(DiffModifies::STYLE);
            diff.insert(DiffModifies::LAYOUT);
        }

        if !self.layout.layout.inner_layout_eq(&rect.layout.layout) {
            diff.insert(DiffModifies::STYLE);
            diff.insert(DiffModifies::INNER_LAYOUT);
        }

        if self.accessibility != rect.accessibility {
            diff.insert(DiffModifies::ACCESSIBILITY);
        }

        if self.relative_layer != rect.relative_layer {
            diff.insert(DiffModifies::LAYER);
        }

        if self.event_handlers != rect.event_handlers {
            diff.insert(DiffModifies::EVENT_HANDLERS);
        }

        if self.text_style_data != rect.text_style_data {
            diff.insert(DiffModifies::TEXT_STYLE);
        }

        diff
    }

    fn layout(&'_ self) -> Cow<'_, LayoutData> {
        Cow::Borrowed(&self.layout)
    }

    fn effect(&'_ self) -> Option<Cow<'_, EffectData>> {
        self.effect.as_ref().map(Cow::Borrowed)
    }

    fn style(&'_ self) -> Cow<'_, StyleState> {
        Cow::Borrowed(&self.style)
    }

    fn text_style(&'_ self) -> Cow<'_, TextStyleData> {
        Cow::Borrowed(&self.text_style_data)
    }

    fn accessibility(&'_ self) -> Cow<'_, AccessibilityData> {
        Cow::Borrowed(&self.accessibility)
    }

    fn layer(&self) -> Layer {
        self.relative_layer
    }

    fn events_handlers(&'_ self) -> Option<Cow<'_, FxHashMap<EventName, EventHandlerType>>> {
        Some(Cow::Borrowed(&self.event_handlers))
    }

    fn is_point_inside(&self, context: EventMeasurementContext) -> bool {
        let style = self.style();
        let area = context.layout_node.visible_area();
        let cursor = context.cursor.to_f32();
        let corner_radius = style.corner_radius;
        let mut path = SkPath::new();
        let rounded_rect = self.container_rect(&area, context.scale_factor as f32);
        if corner_radius.smoothing > 0.0 {
            path.add_path(
                &corner_radius.smoothed_path(rounded_rect),
                (area.min_x(), area.min_y()),
                None,
            );
        } else {
            path.add_rrect(rounded_rect, None);
        }
        rounded_rect.contains(SkRect::new(
            cursor.x,
            cursor.y,
            cursor.x + 0.0001,
            cursor.y + 0.0001,
        ))
    }

    fn clip(&self, context: ClipContext) {
        let style = self.style();
        let area = context.visible_area;
        let corner_radius = style.corner_radius.with_scale(context.scale_factor as f32);

        let mut path = SkPath::new();

        let rounded_rect = self.container_rect(area, context.scale_factor as f32);

        if corner_radius.smoothing > 0.0 {
            path.add_path(
                &corner_radius.smoothed_path(rounded_rect),
                (area.min_x(), area.min_y()),
                None,
            );
        } else {
            path.add_rrect(rounded_rect, None);
        }

        context
            .canvas
            .clip_rrect(rounded_rect, ClipOp::Intersect, true);
    }

    fn render(&self, context: RenderContext) {
        let style = self.style();

        let area = context.layout_node.area;
        let corner_radius = style.corner_radius.with_scale(context.scale_factor as f32);

        let mut path = SkPath::new();
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        style.background.apply_to_paint(&mut paint, area);

        // Container
        let rounded_rect = self.container_rect(&area, context.scale_factor as f32);
        if corner_radius.smoothing > 0.0 {
            path.add_path(
                &corner_radius.smoothed_path(rounded_rect),
                (area.min_x(), area.min_y()),
                None,
            );
        } else {
            path.add_rrect(rounded_rect, None);
        }

        context.canvas.draw_path(&path, &paint);

        // Shadows
        for shadow in style.shadows.iter() {
            if shadow.color != Color::TRANSPARENT {
                let shadow = shadow.with_scale(context.scale_factor as f32);

                Self::render_shadow(
                    context.canvas,
                    &mut path,
                    rounded_rect,
                    area,
                    &shadow,
                    &corner_radius,
                );
            }
        }

        // Borders
        for border in style.borders.iter() {
            if border.is_visible() {
                let border = border.with_scale(context.scale_factor as f32);
                let rect = *rounded_rect.rect();
                Self::render_border(context.canvas, rect, &border, &corner_radius);
            }
        }
    }
}

pub struct Rect {
    element: RectElement,
    elements: Vec<Element>,
    key: DiffKey,
}

impl ChildrenExt for Rect {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
}

impl KeyExt for Rect {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl EventHandlersExt for Rect {
    fn get_event_handlers(&mut self) -> &mut FxHashMap<EventName, EventHandlerType> {
        &mut self.element.event_handlers
    }
}

impl AccessibilityExt for Rect {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.element.accessibility
    }
}

impl TextStyleExt for Rect {
    fn get_text_style_data(&mut self) -> &mut TextStyleData {
        &mut self.element.text_style_data
    }
}

impl MaybeExt for Rect {}

impl LayerExt for Rect {
    fn get_layer(&mut self) -> &mut Layer {
        &mut self.element.relative_layer
    }
}

impl LayoutExt for Rect {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.element.layout
    }
}

impl ContainerExt for Rect {}

impl ContainerWithContentExt for Rect {}

impl ScrollableExt for Rect {
    fn get_effect(&mut self) -> &mut EffectData {
        if self.element.effect.is_none() {
            self.element.effect = Some(EffectData::default())
        }

        self.element.effect.as_mut().unwrap()
    }
}

impl InteractiveExt for Rect {
    fn get_effect(&mut self) -> &mut EffectData {
        if self.element.effect.is_none() {
            self.element.effect = Some(EffectData::default())
        }

        self.element.effect.as_mut().unwrap()
    }
}

impl From<Rect> for Element {
    fn from(value: Rect) -> Self {
        Element::Element {
            key: value.key,
            element: Rc::new(value.element),
            elements: value.elements,
        }
    }
}

impl Rect {
    pub fn empty() -> Self {
        Self {
            element: RectElement::default(),
            elements: Vec::default(),
            key: DiffKey::None,
        }
    }

    pub fn try_downcast(element: &dyn ElementExt) -> Option<RectElement> {
        (element as &dyn Any).downcast_ref::<RectElement>().cloned()
    }

    pub fn border(mut self, border: impl Into<Option<Border>>) -> Self {
        if let Some(border) = border.into() {
            self.element.style.borders.push(border);
        }
        self
    }

    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.element.text_style_data.color = Some(color.into());
        self
    }

    pub fn font_size(mut self, font_size: impl Into<FontSize>) -> Self {
        self.element.text_style_data.font_size = Some(font_size.into());
        self
    }

    pub fn shadow(mut self, shadow: impl Into<Shadow>) -> Self {
        self.element.style.shadows.push(shadow.into());
        self
    }

    pub fn overflow<S: Into<Overflow>>(mut self, overflow: S) -> Self {
        self.element
            .effect
            .get_or_insert_with(Default::default)
            .overflow = overflow.into();
        self
    }

    pub fn rotate<R: Into<Option<f32>>>(mut self, rotation: R) -> Self {
        self.element
            .effect
            .get_or_insert_with(Default::default)
            .rotation = rotation.into();
        self
    }

    pub fn background<S: Into<Color>>(mut self, background: S) -> Self {
        self.element.style.background = Fill::Color(background.into());
        self
    }

    pub fn background_conic_gradient<S: Into<ConicGradient>>(mut self, background: S) -> Self {
        self.element.style.background = Fill::ConicGradient(Box::new(background.into()));
        self
    }

    pub fn background_linear_gradient<S: Into<LinearGradient>>(mut self, background: S) -> Self {
        self.element.style.background = Fill::LinearGradient(Box::new(background.into()));
        self
    }

    pub fn background_radial_gradient<S: Into<RadialGradient>>(mut self, background: S) -> Self {
        self.element.style.background = Fill::RadialGradient(Box::new(background.into()));
        self
    }

    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.element.style.corner_radius = corner_radius.into();
        self
    }

    pub fn scale(mut self, scale: impl Into<Scale>) -> Self {
        self.element
            .effect
            .get_or_insert_with(Default::default)
            .scale = Some(scale.into());
        self
    }

    pub fn opacity(mut self, opacity: impl Into<f32>) -> Self {
        self.element
            .effect
            .get_or_insert_with(Default::default)
            .opacity = Some(opacity.into());
        self
    }
}
