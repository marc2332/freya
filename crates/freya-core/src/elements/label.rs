//! Draw text with [label()]. Its a simplified version of [crate::elements::paragraph].

use std::{
    any::Any,
    borrow::Cow,
    rc::Rc,
};

use freya_engine::prelude::{
    ClipOp,
    FontStyle,
    ParagraphBuilder,
    ParagraphStyle,
    SkParagraph,
    SkRect,
    TextStyle,
};
use rustc_hash::FxHashMap;
use torin::prelude::Size2D;

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
        ContainerExt,
        EventHandlersExt,
        KeyExt,
        LayerExt,
        LayoutExt,
        MaybeExt,
        Span,
        TextAlign,
        TextStyleExt,
    },
    text_cache::CachedParagraph,
    tree::DiffModifies,
};

/// Draw text with [label()]. Its a simplified version of [crate::elements::paragraph].
///
/// See the available methods in [Label].
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     label().text("Hello, world!").font_size(16.0)
/// }
/// ```
pub fn label() -> Label {
    Label {
        key: DiffKey::None,
        element: LabelElement::default(),
    }
}

impl From<&str> for Element {
    fn from(value: &str) -> Self {
        label().text(value.to_string()).into()
    }
}

impl From<String> for Element {
    fn from(value: String) -> Self {
        label().text(value).into()
    }
}

pub enum TextWidth {
    Fit,
    Max,
}

#[derive(PartialEq, Clone)]
pub struct LabelElement {
    pub text: Cow<'static, str>,
    pub accessibility: AccessibilityData,
    pub text_style_data: TextStyleData,
    pub layout: LayoutData,
    pub event_handlers: FxHashMap<EventName, EventHandlerType>,
    pub max_lines: Option<usize>,
    pub line_height: Option<f32>,
    pub relative_layer: Layer,
}

impl Default for LabelElement {
    fn default() -> Self {
        let mut accessibility = AccessibilityData::default();
        accessibility.builder.set_role(accesskit::Role::Label);
        Self {
            text: Default::default(),
            accessibility,
            text_style_data: Default::default(),
            layout: Default::default(),
            event_handlers: Default::default(),
            max_lines: None,
            line_height: None,
            relative_layer: Layer::default(),
        }
    }
}

impl ElementExt for LabelElement {
    fn changed(&self, other: &Rc<dyn ElementExt>) -> bool {
        let Some(label) = (other.as_ref() as &dyn Any).downcast_ref::<LabelElement>() else {
            return false;
        };
        self != label
    }

    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(label) = (other.as_ref() as &dyn Any).downcast_ref::<LabelElement>() else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if self.text != label.text {
            diff.insert(DiffModifies::STYLE);
            diff.insert(DiffModifies::LAYOUT);
        }

        if self.accessibility != label.accessibility {
            diff.insert(DiffModifies::ACCESSIBILITY);
        }

        if self.relative_layer != label.relative_layer {
            diff.insert(DiffModifies::LAYER);
        }

        if self.text_style_data != label.text_style_data
            || self.line_height != label.line_height
            || self.max_lines != label.max_lines
        {
            diff.insert(DiffModifies::TEXT_STYLE);
            diff.insert(DiffModifies::LAYOUT);
        }
        if self.layout != label.layout {
            diff.insert(DiffModifies::LAYOUT);
        }

        if self.event_handlers != label.event_handlers {
            diff.insert(DiffModifies::EVENT_HANDLERS);
        }

        diff
    }

    fn layout(&'_ self) -> Cow<'_, LayoutData> {
        Cow::Borrowed(&self.layout)
    }

    fn effect(&'_ self) -> Option<Cow<'_, EffectData>> {
        None
    }

    fn style(&'_ self) -> Cow<'_, StyleState> {
        Cow::Owned(StyleState::default())
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

    fn measure(&self, context: LayoutContext) -> Option<(Size2D, Rc<dyn Any>)> {
        let cached_paragraph = CachedParagraph {
            text_style_state: context.text_style_state,
            spans: &[Span::new(&*self.text)],
            max_lines: None,
            line_height: None,
            width: context.area_size.width,
        };
        let paragraph = context
            .text_cache
            .utilize(context.node_id, &cached_paragraph)
            .unwrap_or_else(|| {
                let mut paragraph_style = ParagraphStyle::default();
                let mut text_style = TextStyle::default();

                let mut font_families = context.text_style_state.font_families.clone();
                font_families.extend_from_slice(context.fallback_fonts);

                text_style.set_color(context.text_style_state.color);
                text_style.set_font_size(
                    f32::from(context.text_style_state.font_size) * context.scale_factor as f32,
                );
                text_style.set_font_families(&font_families);
                text_style.set_font_style(FontStyle::new(
                    context.text_style_state.font_weight.into(),
                    context.text_style_state.font_width.into(),
                    context.text_style_state.font_slant.into(),
                ));

                if context.text_style_state.text_height.needs_custom_height() {
                    text_style.set_height_override(true);
                    text_style.set_half_leading(true);
                }

                if let Some(line_height) = self.line_height {
                    text_style.set_height_override(true).set_height(line_height);
                }

                for text_shadow in context.text_style_state.text_shadows.iter() {
                    text_style.add_shadow((*text_shadow).into());
                }

                if let Some(ellipsis) = context.text_style_state.text_overflow.get_ellipsis() {
                    paragraph_style.set_ellipsis(ellipsis);
                }

                paragraph_style.set_text_style(&text_style);
                paragraph_style.set_max_lines(self.max_lines);
                paragraph_style.set_text_align(context.text_style_state.text_align.into());

                let mut paragraph_builder =
                    ParagraphBuilder::new(&paragraph_style, context.font_collection);

                paragraph_builder.add_text(&self.text);

                let mut paragraph = paragraph_builder.build();
                paragraph.layout(
                    if self.max_lines == Some(1)
                        && context.text_style_state.text_align == TextAlign::default()
                        && !paragraph_style.ellipsized()
                    {
                        f32::MAX
                    } else {
                        context.area_size.width + 1.0
                    },
                );

                context
                    .text_cache
                    .insert(context.node_id, &cached_paragraph, paragraph)
            });

        let size = Size2D::new(paragraph.longest_line(), paragraph.height());

        Some((size, paragraph))
    }

    fn should_hook_measurement(&self) -> bool {
        true
    }

    fn should_measure_inner_children(&self) -> bool {
        false
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
        let layout_data = context.layout_node.data.as_ref().unwrap();
        let paragraph = layout_data.downcast_ref::<SkParagraph>().unwrap();

        paragraph.paint(context.canvas, context.layout_node.area.origin.to_tuple());
    }
}

impl From<Label> for Element {
    fn from(value: Label) -> Self {
        Element::Element {
            key: value.key,
            element: Rc::new(value.element),
            elements: vec![],
        }
    }
}

impl KeyExt for Label {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl EventHandlersExt for Label {
    fn get_event_handlers(&mut self) -> &mut FxHashMap<EventName, EventHandlerType> {
        &mut self.element.event_handlers
    }
}

impl AccessibilityExt for Label {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.element.accessibility
    }
}

impl TextStyleExt for Label {
    fn get_text_style_data(&mut self) -> &mut TextStyleData {
        &mut self.element.text_style_data
    }
}

impl LayerExt for Label {
    fn get_layer(&mut self) -> &mut Layer {
        &mut self.element.relative_layer
    }
}

impl MaybeExt for Label {}

pub struct Label {
    key: DiffKey,
    element: LabelElement,
}

impl Label {
    pub fn try_downcast(element: &dyn ElementExt) -> Option<LabelElement> {
        (element as &dyn Any)
            .downcast_ref::<LabelElement>()
            .cloned()
    }

    pub fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        let text = text.into();
        self.element.accessibility.builder.set_value(text.clone());
        self.element.text = text;
        self
    }

    pub fn max_lines(mut self, max_lines: impl Into<Option<usize>>) -> Self {
        self.element.max_lines = max_lines.into();
        self
    }

    pub fn line_height(mut self, line_height: impl Into<Option<f32>>) -> Self {
        self.element.line_height = line_height.into();
        self
    }
}

impl LayoutExt for Label {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.element.layout
    }
}

impl ContainerExt for Label {}
