//! Use [svg()] to render SVG in your app.

use std::{
    any::Any,
    borrow::Cow,
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use bytes::Bytes;
use freya_engine::prelude::{
    ClipOp,
    LocalResourceProvider,
    Paint,
    SkRect,
    svg,
};
use rustc_hash::FxHashMap;
use torin::{
    prelude::Size2D,
    size::Size,
};

use crate::{
    data::{
        AccessibilityData,
        EffectData,
        LayoutData,
        StyleState,
        TextStyleData,
    },
    diff_key::DiffKey,
    element::{
        ClipContext,
        Element,
        ElementExt,
        EventHandlerType,
        LayoutContext,
        RenderContext,
    },
    events::name::EventName,
    layers::Layer,
    prelude::{
        AccessibilityExt,
        Color,
        ContainerExt,
        EventHandlersExt,
        KeyExt,
        LayerExt,
        LayoutExt,
        MaybeExt,
    },
    tree::DiffModifies,
};

/// Use [svg()] to render SVG in your app.
///
/// See the available methods in [Svg].
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     svg(Bytes::from_static(include_bytes!("../../../../logo.svg")))
/// }
/// ```
pub fn svg(bytes: Bytes) -> Svg {
    let mut accessibility = AccessibilityData::default();
    accessibility.builder.set_role(accesskit::Role::SvgRoot);

    Svg {
        key: DiffKey::None,
        element: SvgElement {
            accessibility,
            layout: LayoutData::default(),
            event_handlers: HashMap::default(),
            bytes,
            effect: None,
            color: Color::BLACK,
            stroke: None,
            fill: None,
            relative_layer: Layer::default(),
        },
    }
}

#[derive(PartialEq, Clone)]
pub struct SvgElement {
    pub accessibility: AccessibilityData,
    pub layout: LayoutData,
    pub event_handlers: FxHashMap<EventName, EventHandlerType>,
    pub bytes: Bytes,
    pub color: Color,
    pub stroke: Option<Color>,
    pub fill: Option<Color>,
    pub effect: Option<EffectData>,
    pub relative_layer: Layer,
}

impl ElementExt for SvgElement {
    fn changed(&self, other: &Rc<dyn ElementExt>) -> bool {
        let Some(image) = (other.as_ref() as &dyn Any).downcast_ref::<SvgElement>() else {
            return false;
        };
        self != image
    }

    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(svg) = (other.as_ref() as &dyn Any).downcast_ref::<SvgElement>() else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if self.accessibility != svg.accessibility {
            diff.insert(DiffModifies::ACCESSIBILITY);
        }

        if self.relative_layer != svg.relative_layer {
            diff.insert(DiffModifies::LAYER);
        }

        if self.layout != svg.layout || self.bytes != svg.bytes {
            diff.insert(DiffModifies::LAYOUT);
        }

        if self.color != svg.color || self.stroke != svg.stroke {
            diff.insert(DiffModifies::STYLE);
        }

        if self.effect != svg.effect {
            diff.insert(DiffModifies::EFFECT);
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
        Cow::Owned(StyleState::default())
    }

    fn text_style(&'_ self) -> Cow<'_, TextStyleData> {
        Cow::Owned(TextStyleData::default())
    }

    fn accessibility(&'_ self) -> Cow<'_, AccessibilityData> {
        Cow::Borrowed(&self.accessibility)
    }

    fn layer(&self) -> Layer {
        self.relative_layer
    }

    fn should_measure_inner_children(&self) -> bool {
        false
    }

    fn should_hook_measurement(&self) -> bool {
        true
    }

    fn measure(&self, context: LayoutContext) -> Option<(Size2D, Rc<dyn Any>)> {
        let resource_provider = LocalResourceProvider::new(context.font_manager);
        let svg_dom = svg::Dom::from_bytes(&self.bytes, resource_provider);
        if let Ok(mut svg_dom) = svg_dom {
            svg_dom.set_container_size(context.area_size.to_i32().to_tuple());
            let mut root = svg_dom.root();
            match self.layout.layout.width {
                Size::Pixels(px) => {
                    root.set_width(svg::Length::new(px.get(), svg::LengthUnit::PX));
                }
                Size::Percentage(per) => {
                    root.set_width(svg::Length::new(per.get(), svg::LengthUnit::Percentage));
                }
                Size::Fill => {
                    root.set_width(svg::Length::new(100., svg::LengthUnit::Percentage));
                }
                _ => {}
            }
            match self.layout.layout.height {
                Size::Pixels(px) => {
                    root.set_height(svg::Length::new(px.get(), svg::LengthUnit::PX));
                }
                Size::Percentage(per) => {
                    root.set_height(svg::Length::new(per.get(), svg::LengthUnit::Percentage));
                }
                Size::Fill => {
                    root.set_height(svg::Length::new(100., svg::LengthUnit::Percentage));
                }
                _ => {}
            }
            Some((
                Size2D::new(root.width().value, root.height().value),
                Rc::new(RefCell::new(svg_dom)),
            ))
        } else {
            None
        }
    }

    fn clip(&self, context: ClipContext) {
        let area = context.visible_area;
        context.canvas.clip_rect(
            SkRect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
            ClipOp::Intersect,
            true,
        );
    }

    fn render(&self, context: RenderContext) {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);

        let svg_dom = context
            .layout_node
            .data
            .as_ref()
            .unwrap()
            .downcast_ref::<RefCell<svg::Dom>>()
            .unwrap();
        let svg_dom = svg_dom.borrow();

        let mut root = svg_dom.root();
        context.canvas.save();
        context
            .canvas
            .translate(context.layout_node.visible_area().origin.to_tuple());

        root.set_color(self.color.into());
        if let Some(fill) = self.fill {
            root.set_fill(svg::Paint::from_color(fill.into()));
        }
        if let Some(stroke) = self.stroke {
            root.set_stroke(svg::Paint::from_color(stroke.into()));
        }
        svg_dom.render(context.canvas);
        context.canvas.restore();
    }
}

impl From<Svg> for Element {
    fn from(value: Svg) -> Self {
        Element::Element {
            key: value.key,
            element: Rc::new(value.element),
            elements: vec![],
        }
    }
}

impl KeyExt for Svg {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl EventHandlersExt for Svg {
    fn get_event_handlers(&mut self) -> &mut FxHashMap<EventName, EventHandlerType> {
        &mut self.element.event_handlers
    }
}

impl LayoutExt for Svg {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.element.layout
    }
}

impl ContainerExt for Svg {}

impl AccessibilityExt for Svg {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.element.accessibility
    }
}

impl MaybeExt for Svg {}

impl LayerExt for Svg {
    fn get_layer(&mut self) -> &mut Layer {
        &mut self.element.relative_layer
    }
}

pub struct Svg {
    key: DiffKey,
    element: SvgElement,
}

impl Svg {
    pub fn try_downcast(element: &dyn ElementExt) -> Option<SvgElement> {
        (element as &dyn Any).downcast_ref::<SvgElement>().cloned()
    }

    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.element.color = color.into();
        self
    }

    pub fn fill(mut self, fill: impl Into<Color>) -> Self {
        self.element.fill = Some(fill.into());
        self
    }

    pub fn stroke<R: Into<Option<Color>>>(mut self, stroke: R) -> Self {
        self.element.stroke = stroke.into();
        self
    }

    pub fn rotate<R: Into<Option<f32>>>(mut self, rotation: R) -> Self {
        self.element
            .effect
            .get_or_insert_with(Default::default)
            .rotation = rotation.into();
        self
    }
}
