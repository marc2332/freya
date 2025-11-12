use std::{
    any::Any,
    borrow::Cow,
    cell::RefCell,
    fmt::Display,
    rc::Rc,
};

use freya_engine::prelude::{
    FontStyle,
    Paint,
    PaintStyle,
    ParagraphBuilder,
    ParagraphStyle,
    RectHeightStyle,
    RectWidthStyle,
    SkParagraph,
    SkRect,
    TextStyle,
};
use rustc_hash::FxHashMap;
use torin::prelude::Size2D;

use crate::{
    data::{
        AccessibilityData,
        CursorStyleData,
        EffectData,
        LayoutData,
        StyleState,
        TextStyleData,
        TextStyleState,
    },
    diff_key::DiffKey,
    element::{
        Element,
        ElementExt,
        EventHandlerType,
        LayoutContext,
        RenderContext,
    },
    events::name::EventName,
    prelude::{
        AccessibilityExt,
        Color,
        ContainerExt,
        EventHandlersExt,
        KeyExt,
        LayerExt,
        LayoutExt,
        MaybeExt,
        TextAlign,
        TextStyleExt,
    },
    text_cache::CachedParagraph,
    tree::DiffModifies,
};
pub struct ParagraphHolderInner {
    pub paragraph: Rc<SkParagraph>,
    pub scale_factor: f64,
}

#[derive(Clone)]
pub struct ParagraphHolder(pub Rc<RefCell<Option<ParagraphHolderInner>>>);

impl PartialEq for ParagraphHolder {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Default for ParagraphHolder {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(None)))
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct ParagraphElement {
    pub layout: LayoutData,
    pub spans: Vec<Span<'static>>,
    pub accessibility: AccessibilityData,
    pub text_style_data: TextStyleData,
    pub cursor_style_data: CursorStyleData,
    pub event_handlers: FxHashMap<EventName, EventHandlerType>,
    pub sk_paragraph: ParagraphHolder,
    pub cursor_index: Option<usize>,
    pub highlights: Vec<(usize, usize)>,
    pub max_lines: Option<usize>,
    pub line_height: Option<f32>,
    pub relative_layer: i16,
}

impl Display for ParagraphElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .spans
                .iter()
                .map(|s| s.text.clone())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

impl ElementExt for ParagraphElement {
    fn changed(&self, other: &Rc<dyn ElementExt>) -> bool {
        let Some(paragraph) = (other.as_ref() as &dyn Any).downcast_ref::<ParagraphElement>()
        else {
            return false;
        };
        self != paragraph
    }

    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(paragraph) = (other.as_ref() as &dyn Any).downcast_ref::<ParagraphElement>()
        else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if self.spans != paragraph.spans {
            diff.insert(DiffModifies::STYLE);
            diff.insert(DiffModifies::LAYOUT);
        }

        if self.accessibility != paragraph.accessibility {
            diff.insert(DiffModifies::ACCESSIBILITY);
        }

        if self.relative_layer != paragraph.relative_layer {
            diff.insert(DiffModifies::LAYER);
        }

        if self.text_style_data != paragraph.text_style_data {
            diff.insert(DiffModifies::STYLE);
        }

        if self.event_handlers != paragraph.event_handlers {
            diff.insert(DiffModifies::EVENT_HANDLERS);
        }

        if self.cursor_index != paragraph.cursor_index || self.highlights != paragraph.highlights {
            diff.insert(DiffModifies::STYLE);
        }

        if self.text_style_data != paragraph.text_style_data
            || self.line_height != paragraph.line_height
            || self.max_lines != paragraph.max_lines
        {
            diff.insert(DiffModifies::TEXT_STYLE);
            diff.insert(DiffModifies::LAYOUT);
        }

        if self.layout != paragraph.layout {
            diff.insert(DiffModifies::STYLE);
            diff.insert(DiffModifies::LAYOUT);
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

    fn relative_layer(&self) -> i16 {
        self.relative_layer
    }

    fn measure(&self, context: LayoutContext) -> Option<(Size2D, Rc<dyn Any>)> {
        let cached_paragraph = CachedParagraph {
            text_style_state: context.text_style_state,
            spans: &self.spans,
            max_lines: self.max_lines,
            line_height: self.line_height,
            width: context.area_size.width,
        };
        let paragraph = context
            .text_cache
            .get(context.node_id, &cached_paragraph)
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

                for span in &self.spans {
                    let text_style_state =
                        TextStyleState::from_data(context.text_style_state, &span.text_style_data);
                    let mut text_style = TextStyle::new();
                    let mut font_families = context.text_style_state.font_families.clone();
                    font_families.extend_from_slice(context.fallback_fonts);

                    for text_shadow in text_style_state.text_shadows.iter() {
                        text_style.add_shadow((*text_shadow).into());
                    }

                    text_style.set_color(text_style_state.color);
                    text_style.set_font_size(
                        f32::from(text_style_state.font_size) * context.scale_factor as f32,
                    );
                    text_style.set_font_families(&font_families);
                    paragraph_builder.push_style(&text_style);
                    paragraph_builder.add_text(&span.text);
                }

                let mut paragraph = paragraph_builder.build();
                paragraph.layout(
                    if self.max_lines == Some(1)
                        && context.text_style_state.text_align == TextAlign::Start
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

        self.sk_paragraph
            .0
            .borrow_mut()
            .replace(ParagraphHolderInner {
                paragraph,
                scale_factor: context.scale_factor,
            });

        Some((size, Rc::new(())))
    }

    fn should_hook_measurement(&self) -> bool {
        true
    }

    fn should_measure_inner_children(&self) -> bool {
        false
    }

    fn events_handlers(&'_ self) -> Option<Cow<'_, FxHashMap<EventName, EventHandlerType>>> {
        Some(Cow::Borrowed(&self.event_handlers))
    }

    fn render(&self, context: RenderContext) {
        let paragraph = self.sk_paragraph.0.borrow();
        let ParagraphHolderInner { paragraph, .. } = paragraph.as_ref().unwrap();
        let area = context.layout_node.visible_area();

        // Draw highlights
        for (from, to) in self.highlights.iter() {
            let (from, to) = { if from < to { (from, to) } else { (to, from) } };
            let rects = paragraph.get_rects_for_range(
                *from..*to,
                RectHeightStyle::Tight,
                RectWidthStyle::Tight,
            );

            let mut highlights_paint = Paint::default();
            highlights_paint.set_anti_alias(true);
            highlights_paint.set_style(PaintStyle::Fill);
            highlights_paint.set_color(self.cursor_style_data.highlight_color);

            // TODO: Add a expanded option for highlights and cursor

            for rect in rects {
                let rect = SkRect::new(
                    area.min_x() + rect.rect.left,
                    area.min_y() + rect.rect.top,
                    area.min_x() + rect.rect.right,
                    area.min_y() + rect.rect.bottom,
                );
                context.canvas.draw_rect(rect, &highlights_paint);
            }
        }

        // Draw text
        paragraph.paint(context.canvas, area.origin.to_tuple());

        // Draw cursor
        if let Some(cursor_index) = self.cursor_index {
            let cursor_rects = paragraph.get_rects_for_range(
                cursor_index..cursor_index + 1,
                RectHeightStyle::Tight,
                RectWidthStyle::Tight,
            );
            if let Some(cursor_rect) = cursor_rects.first().map(|text| text.rect).or_else(|| {
                // Show the cursor at the end of the text if possible
                let text_len = paragraph
                    .get_glyph_position_at_coordinate((f32::MAX, f32::MAX))
                    .position as usize;
                let last_rects = paragraph.get_rects_for_range(
                    (text_len - 1)..text_len,
                    RectHeightStyle::Tight,
                    RectWidthStyle::Tight,
                );

                if let Some(last_rect) = last_rects.first() {
                    let mut caret = last_rect.rect;
                    caret.left = caret.right;
                    Some(caret)
                } else {
                    None
                }
            }) {
                let cursor_rect = SkRect::new(
                    area.min_x() + cursor_rect.left,
                    area.min_y() + cursor_rect.top,
                    area.min_x() + cursor_rect.left + 2.,
                    area.min_y() + cursor_rect.bottom,
                );

                let mut paint = Paint::default();
                paint.set_anti_alias(true);
                paint.set_style(PaintStyle::Fill);
                paint.set_color(self.cursor_style_data.color);

                context.canvas.draw_rect(cursor_rect, &paint);
            }
        }
    }
}

impl From<Paragraph> for Element {
    fn from(value: Paragraph) -> Self {
        Element::Element {
            key: value.key,
            element: Rc::new(value.element),
            elements: vec![],
        }
    }
}

impl KeyExt for Paragraph {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl EventHandlersExt for Paragraph {
    fn get_event_handlers(&mut self) -> &mut FxHashMap<EventName, EventHandlerType> {
        &mut self.element.event_handlers
    }
}

impl MaybeExt for Paragraph {}

impl LayerExt for Paragraph {
    fn get_layer(&mut self) -> &mut i16 {
        &mut self.element.relative_layer
    }
}

pub struct Paragraph {
    key: DiffKey,
    element: ParagraphElement,
}

/// [paragraph] makes it possible to render rich text with different styles. Its a more personalizable api than [crate::elements::label].
///
/// See the available methods in [Paragraph].
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     paragraph()
///         .span(Span::new("Hello").font_size(24.0))
///         .span(Span::new("World").font_size(16.0))
///         .into()
/// }
/// ```
pub fn paragraph() -> Paragraph {
    Paragraph {
        key: DiffKey::None,
        element: ParagraphElement::default(),
    }
}

impl LayoutExt for Paragraph {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.element.layout
    }
}

impl ContainerExt for Paragraph {}

impl AccessibilityExt for Paragraph {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.element.accessibility
    }
}

impl TextStyleExt for Paragraph {
    fn get_text_style_data(&mut self) -> &mut TextStyleData {
        &mut self.element.text_style_data
    }
}

impl Paragraph {
    pub fn try_downcast(element: &dyn ElementExt) -> Option<ParagraphElement> {
        (element as &dyn Any)
            .downcast_ref::<ParagraphElement>()
            .cloned()
    }

    pub fn spans_iter(mut self, spans: impl Iterator<Item = Span<'static>>) -> Self {
        let spans = spans.collect::<Vec<Span>>();
        // TODO: Accessible paragraphs
        // self.element.accessibility.builder.set_value(text.clone());
        self.element.spans.extend(spans);
        self
    }

    pub fn span(mut self, span: impl Into<Span<'static>>) -> Self {
        let span = span.into();
        // TODO: Accessible paragraphs
        // self.element.accessibility.builder.set_value(text.clone());
        self.element.spans.push(span);
        self
    }

    pub fn cursor_color(mut self, cursor_color: impl Into<Color>) -> Self {
        self.element.cursor_style_data.color = cursor_color.into();
        self
    }

    pub fn highlight_color(mut self, highlight_color: impl Into<Color>) -> Self {
        self.element.cursor_style_data.highlight_color = highlight_color.into();
        self
    }

    pub fn holder(mut self, holder: ParagraphHolder) -> Self {
        self.element.sk_paragraph = holder;
        self
    }

    pub fn cursor_index(mut self, cursor_index: impl Into<Option<usize>>) -> Self {
        self.element.cursor_index = cursor_index.into();
        self
    }

    pub fn highlights(mut self, highlights: impl Into<Option<Vec<(usize, usize)>>>) -> Self {
        if let Some(highlights) = highlights.into() {
            self.element.highlights = highlights;
        }
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

#[derive(Clone, PartialEq, Hash)]
pub struct Span<'a> {
    pub text_style_data: TextStyleData,
    pub text: Cow<'a, str>,
}

impl From<&'static str> for Span<'static> {
    fn from(text: &'static str) -> Self {
        Span {
            text_style_data: TextStyleData::default(),
            text: text.into(),
        }
    }
}

impl From<String> for Span<'static> {
    fn from(text: String) -> Self {
        Span {
            text_style_data: TextStyleData::default(),
            text: text.into(),
        }
    }
}

impl<'a> Span<'a> {
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            text: text.into(),
            text_style_data: TextStyleData::default(),
        }
    }
}

impl<'a> TextStyleExt for Span<'a> {
    fn get_text_style_data(&mut self) -> &mut TextStyleData {
        &mut self.text_style_data
    }
}
