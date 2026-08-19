//! [paragraph()] makes it possible to render rich text with different styles. Its a more customizable API than [crate::elements::label].

use std::{
    any::Any,
    borrow::Cow,
    cell::RefCell,
    fmt::{
        Debug,
        Display,
    },
    rc::Rc,
};

use freya_engine::prelude::{
    BlendMode,
    Canvas,
    FontCollection,
    FontStyle,
    Paint,
    PaintStyle,
    ParagraphBuilder,
    ParagraphStyle,
    PlaceholderAlignment,
    PlaceholderStyle,
    RectHeightStyle,
    RectWidthStyle,
    SaveLayerRec,
    SkParagraph,
    SkRect,
    TextBaseline,
    TextStyle,
};
use torin::prelude::{
    Area,
    Length,
    Point2D,
    Position,
    PostMeasure,
    Size2D,
};
use unicode_segmentation::UnicodeSegmentation;

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
        EventHandlers,
        IntoElement,
        LayoutContext,
        PostMeasureContext,
        RenderContext,
    },
    elements::rect::rect,
    layers::Layer,
    node_id::NodeId,
    prelude::{
        AccessibilityExt,
        ChildrenExt,
        Color,
        ContainerExt,
        ContainerPositionExt,
        EventHandlersExt,
        Fill,
        KeyExt,
        LayerExt,
        LayoutExt,
        MaybeExt,
        TextAlign,
        TextStyleExt,
        VerticalAlign,
    },
    style::cursor::{
        CursorMode,
        CursorStyle,
    },
    text_cache::CachedParagraph,
    tree::DiffModifies,
};

/// [paragraph()] makes it possible to render rich text with different styles. Its a more customizable API than [crate::elements::label].
///
/// See the available methods in [Paragraph].
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     paragraph()
///         .span(Span::new("Hello").font_size(24.0))
///         .span(Span::new("World").font_size(16.0))
/// }
/// ```
pub fn paragraph() -> Paragraph {
    Paragraph::default()
}

pub struct ParagraphHolderInner {
    pub paragraph: Rc<SkParagraph>,
    pub scale_factor: f64,
}

/// A shared slot that receives the laid-out paragraph, so callers can hit-test and measure
/// text after layout. Pass it to a [`paragraph()`] with [`Paragraph::holder`].
#[derive(Clone)]
pub struct ParagraphHolder(pub Rc<RefCell<Option<ParagraphHolderInner>>>);

impl PartialEq for ParagraphHolder {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Debug for ParagraphHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ParagraphHolder")
    }
}

impl Default for ParagraphHolder {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(None)))
    }
}

/// Marks the order of a [Paragraph]'s content.
#[derive(PartialEq, Clone)]
pub enum ParagraphContent {
    Span,
    Element,
}

#[derive(PartialEq, Clone)]
pub struct ParagraphElement {
    pub layout: LayoutData,
    pub spans: Vec<Span<'static>>,
    pub contents: Vec<ParagraphContent>,
    pub accessibility: AccessibilityData,
    pub text_style_data: TextStyleData,
    pub cursor_style_data: CursorStyleData,
    pub event_handlers: EventHandlers,
    pub sk_paragraph: ParagraphHolder,
    pub cursor_index: Option<usize>,
    pub highlights: Vec<(usize, usize)>,
    pub max_lines: Option<usize>,
    pub line_height: Option<f32>,
    pub relative_layer: Layer,
    pub cursor_style: CursorStyle,
    pub cursor_mode: CursorMode,
    pub vertical_align: VerticalAlign,
}

impl Default for ParagraphElement {
    fn default() -> Self {
        let mut accessibility = AccessibilityData::default();
        accessibility.builder.set_role(accesskit::Role::Paragraph);
        Self {
            layout: Default::default(),
            spans: Default::default(),
            contents: Default::default(),
            accessibility,
            text_style_data: Default::default(),
            cursor_style_data: Default::default(),
            event_handlers: Default::default(),
            sk_paragraph: Default::default(),
            cursor_index: Default::default(),
            highlights: Default::default(),
            max_lines: Default::default(),
            line_height: Default::default(),
            relative_layer: Default::default(),
            cursor_style: CursorStyle::default(),
            cursor_mode: CursorMode::default(),
            vertical_align: VerticalAlign::default(),
        }
    }
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

        if self.spans != paragraph.spans || self.contents != paragraph.contents {
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

        if self.cursor_index != paragraph.cursor_index
            || self.highlights != paragraph.highlights
            || self.cursor_mode != paragraph.cursor_mode
            || self.cursor_style != paragraph.cursor_style
            || self.cursor_style_data != paragraph.cursor_style_data
            || self.vertical_align != paragraph.vertical_align
        {
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

    fn is_transparent(&self) -> bool {
        false
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
            .utilize(context.node_id, &cached_paragraph)
            .unwrap_or_else(|| {
                let width = if self.max_lines == Some(1)
                    && context.text_style_state.text_align == TextAlign::default()
                    && context
                        .text_style_state
                        .text_overflow
                        .get_ellipsis()
                        .is_none()
                {
                    f32::MAX
                } else {
                    context.area_size.width + 1.0
                };

                let paragraph = self.build_paragraph(
                    context.text_style_state,
                    context.fallback_fonts,
                    context.scale_factor,
                    context.font_collection,
                    width,
                    &[],
                );
                context
                    .text_cache
                    .insert(context.node_id, &cached_paragraph, paragraph)
            });

        let size = Size2D::new(paragraph.longest_line(), paragraph.height()).max(Size2D::zero());

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
        self.has_inline_content()
    }

    fn needs_post_measure(&self) -> bool {
        self.has_inline_content()
    }

    fn post_measure(&self, context: PostMeasureContext) -> PostMeasure<NodeId> {
        if context.children.is_empty() {
            return PostMeasure::default();
        }

        let placeholders: Vec<Size2D> = context
            .children
            .iter()
            .map(|child| {
                context
                    .layout
                    .get(child)
                    .map(|node| node.area.size)
                    .unwrap()
            })
            .collect();

        let width = self
            .sk_paragraph
            .0
            .borrow()
            .as_ref()
            .map(|holder| holder.paragraph.max_width())
            .unwrap();

        let paragraph = self.build_paragraph(
            context.text_style_state,
            context.fallback_fonts,
            context.scale_factor,
            context.font_collection,
            width,
            &placeholders,
        );
        let rects = paragraph.get_rects_for_placeholders();
        let paragraph_height = paragraph.height();
        // The size with the placeholders in place, so the node is sized around the inline children.
        let content_size = Size2D::new(paragraph.longest_line(), paragraph_height);

        self.sk_paragraph
            .0
            .borrow_mut()
            .replace(ParagraphHolderInner {
                paragraph: Rc::new(paragraph),
                scale_factor: context.scale_factor,
            });

        let visible_area = context.node_layout.visible_area();
        let vertical_offset = match self.vertical_align {
            VerticalAlign::Start => 0.0,
            VerticalAlign::Center => (visible_area.height() - paragraph_height).max(0.0) / 2.0,
        };
        let origin = visible_area.origin;

        let mut offsets = Vec::new();
        let mut hidden_children = Vec::new();
        for (index, child_id) in context.children.iter().enumerate() {
            let Some(current) = context.layout.get(child_id).map(|node| node.area.origin) else {
                continue;
            };
            match rects.get(index) {
                Some(rect) => {
                    let offset_x = origin.x + rect.rect.left - current.x;
                    let offset_y = origin.y + vertical_offset + rect.rect.top - current.y;
                    offsets.push((*child_id, Length::new(offset_x), Length::new(offset_y)));
                }
                // Children whose placeholder got cut off by max_lines or ellipsis are hidden.
                None => hidden_children.push(*child_id),
            }
        }

        PostMeasure {
            content_size: Some(content_size),
            offsets,
            hidden_children,
        }
    }

    fn events_handlers(&'_ self) -> Option<Cow<'_, EventHandlers>> {
        Some(Cow::Borrowed(&self.event_handlers))
    }

    fn render(&self, context: RenderContext) {
        let paragraph = self.sk_paragraph.0.borrow();
        let ParagraphHolderInner { paragraph, .. } = paragraph.as_ref().unwrap();
        let visible_area = context.layout_node.visible_area();

        let cursor_area = match self.cursor_mode {
            CursorMode::Fit => visible_area,
            CursorMode::Expanded => context.layout_node.area,
        };

        let paragraph_height = paragraph.height();
        let area_height = visible_area.height();
        let vertical_offset = match self.vertical_align {
            VerticalAlign::Start => 0.0,
            VerticalAlign::Center => (area_height - paragraph_height).max(0.0) / 2.0,
        };

        let cursor_vertical_offset = match self.cursor_mode {
            CursorMode::Fit => vertical_offset,
            CursorMode::Expanded => 0.0,
        };
        let cursor_vertical_size_offset = match self.cursor_mode {
            CursorMode::Fit => 0.,
            CursorMode::Expanded => vertical_offset * 2.,
        };

        let to_cursor_area = |rect: SkRect| {
            SkRect::new(
                cursor_area.min_x() + rect.left,
                cursor_area.min_y() + rect.top + cursor_vertical_offset,
                cursor_area.min_x() + rect.right,
                cursor_area.min_y() + rect.bottom + cursor_vertical_size_offset,
            )
        };

        // Draw highlights
        for (from, to) in self.highlights.iter() {
            if from == to {
                continue;
            }
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

            if rects.is_empty() && *from == 0 {
                let caret_rect =
                    paragraph.cursor_rect(&self.text(), *from, context.text_style_state.text_align);
                context
                    .canvas
                    .draw_rect(to_cursor_area(caret_rect), &highlights_paint);
            }

            for rect in rects {
                let mut rect = rect.rect;
                rect.right = rect.right.max(6.);
                context
                    .canvas
                    .draw_rect(to_cursor_area(rect), &highlights_paint);
            }
        }

        let visible_highlights = self
            .highlights
            .iter()
            .any(|highlight| highlight.0 != highlight.1);

        let mut cursor_paint = Paint::default();
        cursor_paint.set_anti_alias(true);
        cursor_paint.set_style(PaintStyle::Fill);
        cursor_paint.set_color(self.cursor_style_data.color);

        // Draw block cursor
        if let Some(cursor_index) = self.cursor_index
            && self.cursor_style == CursorStyle::Block
        {
            let mut cursor_rect = paragraph.cursor_rect(
                &self.text(),
                cursor_index,
                context.text_style_state.text_align,
            );
            let width = (cursor_rect.right - cursor_rect.left).max(6.0);
            cursor_rect.right = cursor_rect.left + width;
            context
                .canvas
                .draw_rect(to_cursor_area(cursor_rect), &cursor_paint);
        }

        // Draw text
        paint_paragraph_with_fill(
            paragraph,
            context.canvas,
            Point2D::new(visible_area.min_x(), visible_area.min_y() + vertical_offset),
            &context.text_style_state.color,
        );

        // Draw cursor
        if let Some(cursor_index) = self.cursor_index
            && !visible_highlights
            && self.cursor_style != CursorStyle::Block
        {
            let mut cursor_rect = paragraph.cursor_rect(
                &self.text(),
                cursor_index,
                context.text_style_state.text_align,
            );
            match self.cursor_style {
                CursorStyle::Underline => cursor_rect.top = cursor_rect.bottom - 2.,
                _ => cursor_rect.right = cursor_rect.left + 2.,
            }
            context
                .canvas
                .draw_rect(to_cursor_area(cursor_rect), &cursor_paint);
        }
    }
}

impl ParagraphElement {
    fn has_inline_content(&self) -> bool {
        self.contents
            .iter()
            .any(|content| matches!(content, ParagraphContent::Element))
    }

    /// The paragraph text as Skia indexes it, with inline children as placeholder characters.
    fn text(&self) -> String {
        let mut text = String::new();
        let mut spans = self.spans.iter();
        for content in &self.contents {
            match content {
                ParagraphContent::Span => {
                    if let Some(span) = spans.next() {
                        text.push_str(&span.text);
                    }
                }
                ParagraphContent::Element => text.push('\u{FFFC}'),
            }
        }
        text
    }

    /// Builds the Skia paragraph from the content, reserving a placeholder (sized from
    /// `placeholders`, in order) for each inline child, laid out against `width`.
    fn build_paragraph(
        &self,
        text_style_state: &TextStyleState,
        fallback_fonts: &[Cow<'static, str>],
        scale_factor: f64,
        font_collection: &FontCollection,
        width: f32,
        placeholders: &[Size2D],
    ) -> SkParagraph {
        let mut paragraph_style = ParagraphStyle::default();

        if let Some(ellipsis) = text_style_state.text_overflow.get_ellipsis() {
            paragraph_style.set_ellipsis(ellipsis);
        }

        paragraph_style.set_text_style(&base_text_style(
            text_style_state,
            fallback_fonts,
            scale_factor,
            self.line_height,
        ));
        paragraph_style.set_max_lines(self.max_lines);
        paragraph_style.set_text_align(text_style_state.text_align.into());

        let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);

        let mut spans = self.spans.iter();
        let mut placeholders = placeholders.iter();
        for content in &self.contents {
            match content {
                ParagraphContent::Span => {
                    let Some(span) = spans.next() else { continue };
                    paragraph_builder.push_style(&span_text_style(
                        text_style_state,
                        fallback_fonts,
                        scale_factor,
                        span,
                        self.line_height,
                    ));
                    paragraph_builder.add_text(&span.text);
                }
                ParagraphContent::Element => {
                    let Some(size) = placeholders.next() else {
                        continue;
                    };
                    paragraph_builder.add_placeholder(&PlaceholderStyle::new(
                        size.width,
                        size.height,
                        PlaceholderAlignment::Middle,
                        TextBaseline::Alphabetic,
                        0.0,
                    ));
                }
            }
        }

        let mut paragraph = paragraph_builder.build();
        paragraph.layout(width);
        paragraph
    }
}

pub trait ParagraphCursorExt {
    /// Area of the character at `cursor_index`, if there is any.
    fn measured_cursor_rect(&self, text: &str, cursor_index: usize) -> Option<SkRect>;

    /// Area of the character at `cursor_index`.
    fn cursor_rect(&self, text: &str, cursor_index: usize, text_align: TextAlign) -> SkRect;

    /// UTF-16 index of the glyph at `point`, snapped back into the visual line the point lands on.
    fn cursor_index_at_point(&self, point: (f64, f64)) -> usize;
}

impl ParagraphCursorExt for SkParagraph {
    fn measured_cursor_rect(&self, text: &str, cursor_index: usize) -> Option<SkRect> {
        let mut cluster = 0..0;
        let mut cluster_byte_index = 0;
        for (byte_index, grapheme) in text.grapheme_indices(true) {
            cluster = cluster.end..cluster.end + grapheme.encode_utf16().count();
            cluster_byte_index = byte_index;
            if cluster.end > cursor_index {
                break;
            }
        }

        if !cluster.is_empty() {
            if cluster.end <= cursor_index
                && let Some(cluster_line) = self.get_line_number_at(cluster_byte_index)
                && let Some(line) = self.get_line_metrics_at(cluster_line + 1)
            {
                let left = line.left as f32;
                let top = (line.baseline - line.ascent) as f32;
                let bottom = (line.baseline + line.descent) as f32;
                return Some(SkRect::new(left, top, left, bottom));
            }

            let rects = self.get_rects_for_range(
                cluster.clone(),
                RectHeightStyle::Tight,
                RectWidthStyle::Tight,
            );
            if let Some(rect) = rects.first() {
                let mut rect = rect.rect;
                if cluster.end <= cursor_index {
                    rect.left = rect.right;
                }
                return Some(rect);
            }
        }

        let line = self.get_line_metrics_at(0)?;
        let left = line.left as f32;

        Some(SkRect::new(left, 0., left + 6., line.height as f32))
    }

    fn cursor_rect(&self, text: &str, cursor_index: usize, text_align: TextAlign) -> SkRect {
        if let Some(rect) = self.measured_cursor_rect(text, cursor_index) {
            return rect;
        }

        // In case of no rect we just center the cursor
        let left = match text_align {
            TextAlign::Center => self.max_width() / 2.,
            TextAlign::Right | TextAlign::End => self.max_width() - 6.,
            _ => 0.,
        };

        SkRect::new(left, 0., left + 6., self.height())
    }

    fn cursor_index_at_point(&self, point: (f64, f64)) -> usize {
        let (mut x, y) = point;
        if let Some(line) = self
            .get_line_metrics()
            .into_iter()
            .find(|line| y < line.baseline + line.descent)
        {
            // Past a soft wrap the caret belongs to the end of the line, not to the next one
            let line_end = line.left + line.width;
            if !line.hard_break && x > line_end {
                x = line_end - 0.5;
            }
        }

        self.get_glyph_position_at_coordinate((x as i32, y as i32))
            .position
            .max(0) as usize
    }
}

impl From<Paragraph> for Element {
    fn from(value: Paragraph) -> Self {
        let elements = value
            .children
            .into_iter()
            .map(|child| {
                rect()
                    .position(Position::new_absolute())
                    .child(child)
                    .into_element()
            })
            .collect();

        Element::Element {
            key: value.key,
            element: Rc::new(value.element),
            elements,
        }
    }
}

/// Builds the paragraph-level base [TextStyle] from the inherited text style state.
fn base_text_style(
    text_style_state: &TextStyleState,
    fallback_fonts: &[Cow<'static, str>],
    scale_factor: f64,
    line_height: Option<f32>,
) -> TextStyle {
    let mut text_style = TextStyle::default();

    let mut font_families = text_style_state.font_families.clone();
    font_families.extend_from_slice(fallback_fonts);

    text_style.set_color(text_style_state.color.as_color().unwrap_or(Color::WHITE));
    text_style.set_font_size(f32::from(text_style_state.font_size) * scale_factor as f32);
    text_style.set_font_families(&font_families);
    text_style.set_font_style(FontStyle::new(
        text_style_state.font_weight.into(),
        text_style_state.font_width.into(),
        text_style_state.font_slant.into(),
    ));

    if text_style_state.text_height.needs_custom_height() {
        text_style.set_height_override(true);
        text_style.set_half_leading(true);
    }

    if let Some(line_height) = line_height {
        text_style.set_height_override(true);
        text_style.set_height(line_height);
    }

    for text_shadow in text_style_state.text_shadows.iter() {
        text_style.add_shadow((*text_shadow).into());
    }

    text_style
}

/// Builds the [TextStyle] for a single [Span], layering its overrides over the inherited state.
fn span_text_style(
    text_style_state: &TextStyleState,
    fallback_fonts: &[Cow<'static, str>],
    scale_factor: f64,
    span: &Span,
    line_height: Option<f32>,
) -> TextStyle {
    let span_style = TextStyleState::from_data(text_style_state, &span.text_style_data);
    let mut text_style = TextStyle::new();
    let mut font_families = text_style_state.font_families.clone();
    font_families.extend_from_slice(fallback_fonts);

    for text_shadow in span_style.text_shadows.iter() {
        text_style.add_shadow((*text_shadow).into());
    }

    text_style.set_color(span_style.color.as_color().unwrap_or(Color::WHITE));
    text_style.set_font_size(f32::from(span_style.font_size) * scale_factor as f32);
    text_style.set_font_families(&font_families);
    text_style.set_font_style(FontStyle::new(
        span_style.font_weight.into(),
        span_style.font_width.into(),
        span_style.font_slant.into(),
    ));
    text_style.set_decoration_type(span_style.text_decoration.into());
    if let Some(line_height) = line_height {
        text_style.set_height_override(true);
        text_style.set_height(line_height);
    }
    text_style
}

/// Paints a paragraph with a [Fill] as the text color. Non-color fills are masked
/// onto the rendered glyph alpha via an offscreen layer + [BlendMode::SrcIn].
pub(crate) fn paint_paragraph_with_fill(
    paragraph: &SkParagraph,
    canvas: &Canvas,
    origin: Point2D,
    fill: &Fill,
) {
    if matches!(fill, Fill::Color(_)) {
        paragraph.paint(canvas, origin.to_tuple());
        return;
    }

    let width = paragraph.longest_line();
    let height = paragraph.height();
    let area = Area::new(origin, Size2D::new(width, height));
    let bounds_rect = SkRect::from_xywh(area.min_x(), area.min_y(), width, height);

    let layer = canvas.save_layer(&SaveLayerRec::default().bounds(&bounds_rect));

    paragraph.paint(canvas, origin.to_tuple());

    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_style(PaintStyle::Fill);
    paint.set_blend_mode(BlendMode::SrcIn);
    fill.apply_to_paint(&mut paint, area);

    canvas.draw_rect(bounds_rect, &paint);

    canvas.restore_to_count(layer);
}

impl KeyExt for Paragraph {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl EventHandlersExt for Paragraph {
    fn get_event_handlers(&mut self) -> &mut EventHandlers {
        &mut self.element.event_handlers
    }
}

impl MaybeExt for Paragraph {}

impl LayerExt for Paragraph {
    fn get_layer(&mut self) -> &mut Layer {
        &mut self.element.relative_layer
    }
}

#[derive(Default, Clone)]
pub struct Paragraph {
    key: DiffKey,
    element: ParagraphElement,
    children: Vec<Element>,
}

impl LayoutExt for Paragraph {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.element.layout
    }
}

impl ContainerExt for Paragraph {}

/// Children added to a [Paragraph] flow inline at the point they were added, each laid out at
/// its own measured size, so give them an explicit `width` and `height`.
impl ChildrenExt for Paragraph {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }

    fn child<C: IntoElement>(mut self, child: C) -> Self {
        self.element.contents.push(ParagraphContent::Element);
        self.children.push(child.into_element());
        self
    }

    fn children(self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        children
            .into_iter()
            .fold(self, |paragraph, child| paragraph.child(child))
    }

    fn maybe_child<C: IntoElement>(self, child: Option<C>) -> Self {
        match child {
            Some(child) => self.child(child),
            None => self,
        }
    }
}

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

    /// Append every [`Span`] yielded by the iterator to the paragraph.
    pub fn spans_iter(mut self, spans: impl Iterator<Item = Span<'static>>) -> Self {
        // TODO: Accessible paragraphs
        // self.element.accessibility.builder.set_value(text.clone());
        for span in spans {
            self.push_span(span);
        }
        self
    }

    /// Append a single [`Span`] of styled text to the paragraph.
    pub fn span(mut self, span: impl Into<Span<'static>>) -> Self {
        // TODO: Accessible paragraphs
        // self.element.accessibility.builder.set_value(text.clone());
        self.push_span(span.into());
        self
    }

    fn push_span(&mut self, span: Span<'static>) {
        self.element.contents.push(ParagraphContent::Span);
        self.element.spans.push(span);
    }

    /// Replace all of the paragraph's cursor style data at once. See [`CursorStyleData`].
    pub fn cursor_style_data(mut self, cursor_style_data: CursorStyleData) -> Self {
        self.element.cursor_style_data = cursor_style_data;
        self
    }

    /// Set the color of the text cursor. See [`Color`].
    pub fn cursor_color(mut self, cursor_color: impl Into<Color>) -> Self {
        self.element.cursor_style_data.color = cursor_color.into();
        self
    }

    /// Set the color used to highlight selected text. See [`Color`].
    pub fn highlight_color(mut self, highlight_color: impl Into<Color>) -> Self {
        self.element.cursor_style_data.highlight_color = highlight_color.into();
        self
    }

    /// Set the shape of the text cursor. See [`CursorStyle`].
    pub fn cursor_style(mut self, cursor_style: impl Into<CursorStyle>) -> Self {
        self.element.cursor_style = cursor_style.into();
        self
    }

    /// Attach a [`ParagraphHolder`] that receives the laid-out paragraph for hit-testing and measurement.
    pub fn holder(mut self, holder: ParagraphHolder) -> Self {
        self.element.sk_paragraph = holder;
        self
    }

    /// Place the text cursor at the given character index. Pass `None` to hide it.
    pub fn cursor_index(mut self, cursor_index: impl Into<Option<usize>>) -> Self {
        self.element.cursor_index = cursor_index.into();
        self
    }

    /// Highlight the given `(start, end)` character ranges, used for text selection.
    pub fn highlights(mut self, highlights: impl Into<Option<Vec<(usize, usize)>>>) -> Self {
        if let Some(highlights) = highlights.into() {
            self.element.highlights = highlights;
        }
        self
    }

    /// Limit the paragraph to at most this many lines, truncating the rest. Pass `None` for no limit.
    pub fn max_lines(mut self, max_lines: impl Into<Option<usize>>) -> Self {
        self.element.max_lines = max_lines.into();
        self
    }

    /// Override the height of each line as a multiple of the font size. Pass `None` for the default.
    pub fn line_height(mut self, line_height: impl Into<Option<f32>>) -> Self {
        self.element.line_height = line_height.into();
        self
    }

    /// Set the cursor mode for the paragraph.
    /// - `CursorMode::Fit`: cursor/highlights use the paragraph's visible_area. VerticalAlign affects cursor positions.
    /// - `CursorMode::Expanded`: cursor/highlights use the paragraph's inner_area. VerticalAlign does NOT affect cursor positions.
    pub fn cursor_mode(mut self, cursor_mode: impl Into<CursorMode>) -> Self {
        self.element.cursor_mode = cursor_mode.into();
        self
    }

    /// Set the vertical alignment for the paragraph text.
    /// This affects how the text is rendered within the paragraph area, but cursor/highlight behavior
    /// depends on the `cursor_mode` setting.
    pub fn vertical_align(mut self, vertical_align: impl Into<VerticalAlign>) -> Self {
        self.element.vertical_align = vertical_align.into();
        self
    }
}

/// A run of text with its own style, used to build a [`paragraph()`].
///
/// Create it with [`Span::new`] (or from a `&str`/`String`) and style it with the
/// [`TextStyleExt`] methods such as [`font_size`](TextStyleExt::font_size) and
/// [`color`](TextStyleExt::color):
///
/// ```
/// # use freya_core::prelude::*;
/// let span = Span::new("Hello").font_size(24.0).color(Color::RED);
/// ```
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
    /// Create a [`Span`] from the given text, with the default text style.
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
