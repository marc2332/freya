use std::{
    borrow::Cow,
    mem,
};

#[cfg(feature = "remote-asset")]
use freya_components::Url;
#[cfg(feature = "remote-asset")]
use freya_components::image_viewer::ImageViewer;
#[cfg(feature = "router")]
use freya_components::link::{
    Link,
    LinkTooltip,
};
use freya_components::{
    define_theme,
    get_theme_or_default,
    table::{
        Table,
        TableBody,
        TableCell,
        TableHead,
        TableRow,
    },
    theming::macros::Preference,
};
use freya_core::{
    elements::rect::Rect,
    prelude::*,
};
use pulldown_cmark::{
    Event,
    HeadingLevel,
    Options,
    Parser,
    Tag,
    TagEnd,
};
use torin::prelude::*;

#[cfg(feature = "code-editor")]
mod code_editor;
#[cfg(feature = "code-editor")]
use code_editor::CodeBlockEditor;

define_theme! {
    %[component]
    pub MarkdownViewer {
        %[fields]
        color: Color,
        color_link: Color,
        background_code: Color,
        color_code: Color,
        background_blockquote: Color,
        border_blockquote: Color,
        background_divider: Color,
        heading_h1: f32,
        heading_h2: f32,
        heading_h3: f32,
        heading_h4: f32,
        heading_h5: f32,
        heading_h6: f32,
        paragraph_size: f32,
        code_font_size: f32,
        table_font_size: f32,
    }
}

fn markdown_theme_preference() -> MarkdownViewerThemePreference {
    MarkdownViewerThemePreference {
        color: Preference::Reference("text_primary"),
        color_link: Preference::Reference("text_highlight"),
        background_code: Preference::Reference("surface_tertiary"),
        color_code: Preference::Reference("text_primary"),
        background_blockquote: Preference::Reference("surface_tertiary"),
        border_blockquote: Preference::Reference("surface_primary"),
        background_divider: Preference::Reference("border"),
        heading_h1: Preference::Specific(32.0),
        heading_h2: Preference::Specific(28.0),
        heading_h3: Preference::Specific(24.0),
        heading_h4: Preference::Specific(20.0),
        heading_h5: Preference::Specific(18.0),
        heading_h6: Preference::Specific(16.0),
        paragraph_size: Preference::Specific(16.0),
        code_font_size: Preference::Specific(14.0),
        table_font_size: Preference::Specific(14.0),
    }
}

/// Markdown viewer component.
///
/// Renders markdown content with support for:
/// - Headings (h1-h6)
/// - Paragraphs
/// - Bold, italic, and strikethrough text
/// - Code (inline and blocks)
/// - Lists (ordered and unordered)
/// - Tables
/// - Images
/// - Links
/// - Blockquotes
/// - Horizontal rules
/// - Custom inline elements (see [`MarkdownViewer::inline_element`])
///
/// With the `code-editor` feature enabled, code blocks are rendered with the
/// `CodeEditor` component for syntax highlighting. Otherwise they fall back to
/// plain monospace text.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     MarkdownViewer::new("# Hello World\n\nThis is **bold** and *italic* text.")
/// }
/// ```
#[derive(PartialEq)]
pub struct MarkdownViewer {
    content: Cow<'static, str>,
    layout: LayoutData,
    key: DiffKey,
    pub(crate) theme: Option<MarkdownViewerThemePartial>,
    inline_element: Option<Callback<String, Option<Element>>>,
    code_editor_font_family: Cow<'static, str>,
    #[cfg(feature = "code-editor")]
    language_resolver: Option<code_editor::LanguageResolver>,
}

impl MarkdownViewer {
    pub fn new(content: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content: content.into(),
            layout: LayoutData::default(),
            key: DiffKey::None,
            theme: None,
            inline_element: None,
            code_editor_font_family: Cow::Borrowed("Jetbrains Mono"),
            #[cfg(feature = "code-editor")]
            language_resolver: None,
        }
    }

    /// Set a handler for custom inline elements.
    ///
    /// Each raw inline HTML tag in a paragraph (for example `<rust-logo/>`) is passed to the
    /// `handler`, which returns the element to inline, or `None` to keep the tag as plain text.
    ///
    /// ```rust
    /// # use freya::prelude::*;
    /// fn app() -> impl IntoElement {
    ///     MarkdownViewer::new("Made with Rust <rust-logo/> btw")
    ///         .inline_element(|html: String| html.starts_with("<rust-logo").then(|| "🦀"))
    /// }
    /// ```
    pub fn inline_element<ReturnedElement: IntoElement + 'static>(
        mut self,
        handler: impl Into<Callback<String, Option<ReturnedElement>>>,
    ) -> Self {
        let handler = handler.into();
        self.inline_element = Some(Callback::new(move |html| {
            handler.call(html).map(IntoElement::into_element)
        }));
        self
    }

    /// Sets the font family used for code blocks. Defaults to `"Jetbrains Mono"`.
    pub fn code_editor_font_family(mut self, font_family: impl Into<Cow<'static, str>>) -> Self {
        self.code_editor_font_family = font_family.into();
        self
    }

    /// Sets a resolver mapping a code block's language to an `EditorLanguage` for highlighting.
    #[cfg(feature = "code-editor")]
    pub fn code_editor_language(
        mut self,
        resolver: impl Into<code_editor::LanguageResolver>,
    ) -> Self {
        self.language_resolver = Some(resolver.into());
        self
    }
}

impl KeyExt for MarkdownViewer {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for MarkdownViewer {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerExt for MarkdownViewer {}

#[allow(dead_code)]
#[derive(Clone)]
enum MarkdownElement {
    Heading {
        level: HeadingLevel,
        spans: Vec<TextSpan>,
    },
    Paragraph {
        content: Vec<Inline>,
    },
    CodeBlock {
        code: String,
        language: Option<String>,
    },
    List(List),
    Image {
        url: String,
        alt: String,
    },
    Link {
        url: String,
        title: Option<String>,
        content: Vec<Inline>,
    },
    Blockquote {
        content: Vec<Inline>,
    },
    Table {
        headers: Vec<Vec<TextSpan>>,
        rows: Vec<Vec<Vec<TextSpan>>>,
    },
    HorizontalRule,
}

/// A markdown list, ordered when `start` is present.
#[derive(Clone)]
struct List {
    start: Option<u64>,
    items: Vec<ListItem>,
}

/// A list item's inline content plus the lists nested under it.
#[derive(Clone)]
struct ListItem {
    content: Vec<Inline>,
    nested_lists: Vec<List>,
}

/// A piece of a paragraph's content: styled text, an image or an inline link flowing within the text.
#[derive(Clone)]
enum Inline {
    Span(TextSpan),
    Image {
        url: String,
        alt: String,
    },
    #[cfg_attr(not(feature = "router"), allow(dead_code))]
    Link {
        url: String,
        title: Option<String>,
        content: Vec<Inline>,
    },
    /// A raw inline HTML tag, resolved at render time by [`MarkdownViewer::inline_element`].
    Html(String),
}

/// Represents styled text spans within markdown.
#[derive(Clone, Debug)]
struct TextSpan {
    text: String,
    bold: bool,
    italic: bool,
    #[allow(dead_code)]
    strikethrough: bool,
    code: bool,
}

impl TextSpan {
    fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            bold: false,
            italic: false,
            strikethrough: false,
            code: false,
        }
    }
}

fn parse_markdown(content: &str) -> Vec<MarkdownElement> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(content, options);
    let mut elements = Vec::new();
    let mut current_spans: Vec<TextSpan> = Vec::new();
    let mut current_content: Vec<Inline> = Vec::new();
    let mut list_stack: Vec<List> = Vec::new();
    let mut item_stack: Vec<ListItem> = Vec::new();

    let mut in_heading: Option<HeadingLevel> = None;
    let mut in_paragraph = false;
    let mut in_code_block = false;
    let mut code_block_content = String::new();
    let mut code_block_language: Option<String> = None;
    let mut in_blockquote = false;
    let mut blockquote_content: Vec<Inline> = Vec::new();

    let mut in_table_cell = false;
    let mut table_headers: Vec<Vec<TextSpan>> = Vec::new();
    let mut table_rows: Vec<Vec<Vec<TextSpan>>> = Vec::new();
    let mut current_table_row: Vec<Vec<TextSpan>> = Vec::new();
    let mut current_cell_spans: Vec<TextSpan> = Vec::new();

    let mut in_link = false;
    let mut link_url: Option<String> = None;
    let mut link_title: Option<String> = None;
    let mut link_content: Vec<Inline> = Vec::new();

    let mut in_image = false;
    let mut image_url = String::new();
    let mut image_title = String::new();
    let mut image_alt = String::new();

    let mut bold = false;
    let mut italic = false;
    let mut strikethrough = false;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    in_heading = Some(level);
                    current_spans.clear();
                }
                Tag::Paragraph => {
                    if in_blockquote {
                        // Paragraphs inside blockquotes
                    } else if !item_stack.is_empty() {
                        // Paragraphs inside list items
                    } else {
                        in_paragraph = true;
                        current_spans.clear();
                        current_content.clear();
                    }
                }
                Tag::CodeBlock(kind) => {
                    in_code_block = true;
                    code_block_content.clear();
                    code_block_language = match kind {
                        pulldown_cmark::CodeBlockKind::Fenced(lang) => {
                            let lang_str = lang.to_string();
                            if lang_str.is_empty() {
                                None
                            } else {
                                Some(lang_str)
                            }
                        }
                        pulldown_cmark::CodeBlockKind::Indented => None,
                    };
                }
                Tag::List(start) => {
                    list_stack.push(List {
                        start,
                        items: Vec::new(),
                    });
                }
                Tag::Item => {
                    item_stack.push(ListItem {
                        content: Vec::new(),
                        nested_lists: Vec::new(),
                    });
                }
                Tag::Strong => bold = true,
                Tag::Emphasis => italic = true,
                Tag::Strikethrough => strikethrough = true,
                Tag::BlockQuote(_) => {
                    in_blockquote = true;
                    blockquote_content.clear();
                }
                Tag::Image {
                    dest_url, title, ..
                } => {
                    in_image = true;
                    image_url = dest_url.to_string();
                    image_title = title.to_string();
                    image_alt.clear();
                }
                Tag::Link {
                    dest_url, title, ..
                } => {
                    in_link = true;
                    link_url = Some(dest_url.to_string());
                    link_title = Some(title.to_string());
                    link_content.clear();
                }
                Tag::Table(_) => {
                    table_headers.clear();
                    table_rows.clear();
                    current_table_row.clear();
                }
                Tag::TableHead => {}
                Tag::TableRow => {
                    current_table_row.clear();
                }
                Tag::TableCell => {
                    in_table_cell = true;
                    current_cell_spans.clear();
                }
                _ => {}
            },
            Event::End(tag_end) => match tag_end {
                TagEnd::Heading(_) => {
                    if let Some(level) = in_heading.take() {
                        elements.push(MarkdownElement::Heading {
                            level,
                            spans: mem::take(&mut current_spans),
                        });
                    }
                }
                TagEnd::Paragraph => {
                    if in_blockquote {
                        blockquote_content.extend(current_spans.drain(..).map(Inline::Span))
                    } else if let Some(item) = item_stack.last_mut() {
                        item.content
                            .extend(current_spans.drain(..).map(Inline::Span))
                    } else if in_paragraph {
                        in_paragraph = false;
                        current_content.extend(current_spans.drain(..).map(Inline::Span));
                        elements.push(MarkdownElement::Paragraph {
                            content: mem::take(&mut current_content),
                        });
                    }
                }
                TagEnd::CodeBlock => {
                    in_code_block = false;
                    elements.push(MarkdownElement::CodeBlock {
                        code: mem::take(&mut code_block_content),
                        language: code_block_language.take(),
                    });
                }
                TagEnd::List(_) => {
                    if let Some(list) = list_stack.pop() {
                        if let Some(item) = item_stack.last_mut() {
                            item.nested_lists.push(list);
                        } else {
                            elements.push(MarkdownElement::List(list));
                        }
                    }
                }
                TagEnd::Item => {
                    if let (Some(item), Some(list)) = (item_stack.pop(), list_stack.last_mut()) {
                        list.items.push(item);
                    }
                }
                TagEnd::Strong => bold = false,
                TagEnd::Emphasis => italic = false,
                TagEnd::Strikethrough => strikethrough = false,
                TagEnd::BlockQuote(_) => {
                    in_blockquote = false;
                    elements.push(MarkdownElement::Blockquote {
                        content: mem::take(&mut blockquote_content),
                    });
                }
                TagEnd::Table => {
                    elements.push(MarkdownElement::Table {
                        headers: mem::take(&mut table_headers),
                        rows: mem::take(&mut table_rows),
                    });
                }
                TagEnd::TableHead => {
                    // TableHead contains cells directly (no TableRow), so save headers here
                    table_headers = mem::take(&mut current_table_row);
                }
                TagEnd::TableRow => {
                    // TableRow only appears in body rows, not in TableHead
                    table_rows.push(mem::take(&mut current_table_row));
                }
                TagEnd::TableCell => {
                    in_table_cell = false;
                    current_table_row.push(mem::take(&mut current_cell_spans));
                }
                TagEnd::Image => {
                    in_image = false;
                    let url = mem::take(&mut image_url);
                    let alt = if image_alt.is_empty() {
                        mem::take(&mut image_title)
                    } else {
                        mem::take(&mut image_alt)
                    };
                    if in_link {
                        link_content.push(Inline::Image { url, alt });
                    } else if in_blockquote {
                        blockquote_content.push(Inline::Image { url, alt });
                    } else if let Some(item) = item_stack.last_mut() {
                        item.content.push(Inline::Image { url, alt });
                    } else if in_paragraph {
                        current_content.extend(current_spans.drain(..).map(Inline::Span));
                        current_content.push(Inline::Image { url, alt });
                    } else {
                        elements.push(MarkdownElement::Image { url, alt });
                    }
                }
                TagEnd::Link => {
                    in_link = false;
                    if let Some(url) = link_url.take() {
                        let title = link_title.take();
                        let content = mem::take(&mut link_content);
                        if in_blockquote {
                            blockquote_content.push(Inline::Link {
                                url,
                                title,
                                content,
                            });
                        } else if let Some(item) = item_stack.last_mut() {
                            item.content.push(Inline::Link {
                                url,
                                title,
                                content,
                            });
                        } else if in_paragraph {
                            current_content.extend(current_spans.drain(..).map(Inline::Span));
                            current_content.push(Inline::Link {
                                url,
                                title,
                                content,
                            });
                        } else {
                            elements.push(MarkdownElement::Link {
                                url,
                                title,
                                content,
                            });
                        }
                    }
                }
                _ => {}
            },
            Event::Text(text) => {
                if in_code_block {
                    code_block_content.push_str(text.trim());
                } else if in_image {
                    image_alt.push_str(&text);
                } else if in_table_cell {
                    let span = TextSpan {
                        text: text.to_string(),
                        bold,
                        italic,
                        strikethrough,
                        code: false,
                    };
                    current_cell_spans.push(span);
                } else {
                    let span = TextSpan {
                        text: text.to_string(),
                        bold,
                        italic,
                        strikethrough,
                        code: false,
                    };
                    if in_link {
                        link_content.push(Inline::Span(span));
                    } else if in_blockquote && !in_paragraph {
                        blockquote_content.push(Inline::Span(span));
                    } else if let Some(item) = item_stack.last_mut()
                        && !in_paragraph
                    {
                        item.content.push(Inline::Span(span));
                    } else {
                        current_spans.push(span);
                    }
                }
            }
            Event::Code(code) => {
                if in_image {
                    image_alt.push_str(&code);
                    continue;
                }
                let span = TextSpan {
                    text: code.to_string(),
                    bold,
                    italic,
                    strikethrough,
                    code: true,
                };
                if in_table_cell {
                    current_cell_spans.push(span);
                } else if in_link {
                    link_content.push(Inline::Span(span));
                } else if in_blockquote {
                    blockquote_content.push(Inline::Span(span));
                } else if let Some(item) = item_stack.last_mut() {
                    item.content.push(Inline::Span(span));
                } else {
                    current_spans.push(span);
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                if in_image {
                    image_alt.push(' ');
                    continue;
                }
                let span = TextSpan::new(" ");
                if in_link {
                    link_content.push(Inline::Span(span));
                } else if in_blockquote {
                    blockquote_content.push(Inline::Span(span));
                } else if let Some(item) = item_stack.last_mut() {
                    item.content.push(Inline::Span(span));
                } else {
                    current_spans.push(span);
                }
            }
            Event::InlineHtml(html) => {
                if in_paragraph && !in_link {
                    current_content.extend(current_spans.drain(..).map(Inline::Span));
                    current_content.push(Inline::Html(html.to_string()));
                }
            }
            Event::Rule => {
                elements.push(MarkdownElement::HorizontalRule);
            }
            _ => {}
        }
    }

    elements
}

/// Build a styled [Span] from a markdown text span.
fn styled_span(span: &TextSpan, text_color: Color, code_color: Color) -> Span<'static> {
    let mut styled = Span::new(span.text.clone());
    if span.bold {
        styled = styled.font_weight(FontWeight::BOLD);
    }
    if span.italic {
        styled = styled.font_slant(FontSlant::Italic);
    }
    if span.code {
        styled.font_family("monospace").color(code_color)
    } else {
        styled.color(text_color)
    }
}

/// Render text spans as a paragraph element.
fn render_spans(
    spans: &[TextSpan],
    base_font_size: f32,
    text_color: Color,
    code_color: Color,
) -> Paragraph {
    paragraph().font_size(base_font_size).spans_iter(
        spans
            .iter()
            .map(|span| styled_span(span, text_color, code_color)),
    )
}

/// Render a list and, recursively, the lists nested under its items.
fn render_list(
    list: &List,
    paragraph_size: f32,
    color: Color,
    color_link: Color,
    color_code: Color,
    inline_element: Option<&Callback<String, Option<Element>>>,
) -> Rect {
    rect()
        .vertical()
        .spacing(4.)
        .padding(Gaps::new(0., 0., 0., 20.))
        .children(list.items.iter().enumerate().map(|(item_idx, item)| {
            rect()
                .key(item_idx)
                .horizontal()
                .cross_align(Alignment::Start)
                .spacing(8.)
                .child(
                    label()
                        .text(match list.start {
                            Some(start) => format!("{}.", start + item_idx as u64),
                            None => "•".to_string(),
                        })
                        .font_size(paragraph_size)
                        .color(color),
                )
                .child(
                    rect()
                        .vertical()
                        .spacing(4.)
                        .child(render_content(
                            &item.content,
                            paragraph_size,
                            color,
                            color_link,
                            color_code,
                            inline_element,
                        ))
                        .children(item.nested_lists.iter().map(|nested_list| {
                            render_list(
                                nested_list,
                                paragraph_size,
                                color,
                                color_link,
                                color_code,
                                inline_element,
                            )
                            .into()
                        })),
                )
                .into()
        }))
}

/// Render a markdown image.
#[cfg(feature = "remote-asset")]
fn render_image(url: &str, alt: &str, text_color: Color) -> Element {
    match url.parse::<Url>() {
        Ok(uri) => ImageViewer::new(uri)
            .a11y_alt(alt)
            .aspect_ratio(AspectRatio::Fit)
            .into(),
        Err(_) => label()
            .text(format!("[Invalid image URL: {}]", url))
            .color(text_color)
            .into(),
    }
}

/// Render a markdown image as its alt text when remote assets are disabled.
#[cfg(not(feature = "remote-asset"))]
fn render_image(_url: &str, alt: &str, text_color: Color) -> Element {
    label()
        .text(format!("[Image: {}]", alt))
        .color(text_color)
        .into()
}

/// Render a paragraph's content, flowing inline links (colored with `link_color`) and images
/// between the text.
fn render_content(
    content: &[Inline],
    base_font_size: f32,
    text_color: Color,
    link_color: Color,
    code_color: Color,
    inline_element: Option<&Callback<String, Option<Element>>>,
) -> Paragraph {
    let mut result = paragraph().font_size(base_font_size);
    for item in content {
        result = match item {
            Inline::Span(span) => result.span(styled_span(span, text_color, code_color)),
            Inline::Image { url, alt } => result.child(render_image(url, alt, text_color)),
            Inline::Html(raw) => {
                match inline_element.and_then(|handler| handler.call(raw.clone())) {
                    Some(element) => result.child(element),
                    None => result.span(Span::new(raw.clone()).color(text_color)),
                }
            }
            #[cfg(feature = "router")]
            Inline::Link {
                url,
                title,
                content,
            } => {
                let mut tooltip = LinkTooltip::Default;
                if let Some(title) = title
                    && !title.is_empty()
                {
                    tooltip = LinkTooltip::Custom(title.clone());
                }
                result.child(
                    Link::new(url.clone())
                        .tooltip(tooltip)
                        .child(render_content(
                            content,
                            base_font_size,
                            link_color,
                            link_color,
                            code_color,
                            inline_element,
                        )),
                )
            }
            #[cfg(not(feature = "router"))]
            Inline::Link { content, .. } => {
                content.iter().fold(result, |paragraph, item| match item {
                    Inline::Span(span) => paragraph.span(styled_span(span, link_color, code_color)),
                    Inline::Image { url, alt } => {
                        paragraph.child(render_image(url, alt, text_color))
                    }
                    _ => paragraph,
                })
            }
        };
    }
    result
}

impl Component for MarkdownViewer {
    fn render(&self) -> impl IntoElement {
        let elements = parse_markdown(&self.content);

        let MarkdownViewerTheme {
            color,
            color_link,
            #[cfg(not(feature = "code-editor"))]
            background_code,
            #[cfg(feature = "code-editor")]
                background_code: _,
            color_code,
            background_blockquote,
            border_blockquote,
            background_divider,
            heading_h1,
            heading_h2,
            heading_h3,
            heading_h4,
            heading_h5,
            heading_h6,
            paragraph_size,
            code_font_size,
            table_font_size,
        } = get_theme_or_default!(
            &self.theme,
            MarkdownViewerThemePreference,
            "markdown_viewer",
            markdown_theme_preference
        );

        let mut container = rect().vertical().layout(self.layout.clone()).spacing(12.);

        for (idx, element) in elements.into_iter().enumerate() {
            let child: Element = match element {
                MarkdownElement::Heading { level, spans } => {
                    let font_size = match level {
                        HeadingLevel::H1 => heading_h1,
                        HeadingLevel::H2 => heading_h2,
                        HeadingLevel::H3 => heading_h3,
                        HeadingLevel::H4 => heading_h4,
                        HeadingLevel::H5 => heading_h5,
                        HeadingLevel::H6 => heading_h6,
                    };
                    render_spans(&spans, font_size, color, color_code)
                        .font_weight(FontWeight::BOLD)
                        .key(idx)
                        .into()
                }
                MarkdownElement::Paragraph { content } => render_content(
                    &content,
                    paragraph_size,
                    color,
                    color_link,
                    color_code,
                    self.inline_element.as_ref(),
                )
                .key(idx)
                .into(),
                MarkdownElement::CodeBlock {
                    code,
                    #[cfg(feature = "code-editor")]
                    language,
                    #[cfg(not(feature = "code-editor"))]
                        language: _,
                } => {
                    #[cfg(feature = "code-editor")]
                    let element = CodeBlockEditor::new(
                        move || Cow::Owned(code.clone()),
                        language,
                        self.language_resolver.clone(),
                        code_font_size,
                        self.code_editor_font_family.clone(),
                    )
                    .key(idx)
                    .into();

                    #[cfg(not(feature = "code-editor"))]
                    let element = rect()
                        .key(idx)
                        .width(Size::fill())
                        .background(background_code)
                        .corner_radius(6.)
                        .padding(Gaps::new_all(12.))
                        .child(
                            label()
                                .text(code)
                                .font_family(self.code_editor_font_family.clone())
                                .font_size(code_font_size)
                                .color(color_code),
                        )
                        .into();

                    element
                }
                MarkdownElement::List(list) => render_list(
                    &list,
                    paragraph_size,
                    color,
                    color_link,
                    color_code,
                    self.inline_element.as_ref(),
                )
                .key(idx)
                .into(),
                MarkdownElement::Image { url, alt } => rect()
                    .key(idx)
                    .child(render_image(&url, &alt, color))
                    .into(),
                #[cfg(feature = "router")]
                MarkdownElement::Link {
                    url,
                    title,
                    content,
                } => {
                    let mut tooltip = LinkTooltip::Default;
                    if let Some(title) = title
                        && !title.is_empty()
                    {
                        tooltip = LinkTooltip::Custom(title);
                    }

                    Link::new(url)
                        .tooltip(tooltip)
                        .child(render_content(
                            &content,
                            paragraph_size,
                            color_link,
                            color_link,
                            color_code,
                            self.inline_element.as_ref(),
                        ))
                        .key(idx)
                        .into()
                }
                #[cfg(not(feature = "router"))]
                MarkdownElement::Link { content, .. } => render_content(
                    &content,
                    paragraph_size,
                    color,
                    color_link,
                    color_code,
                    self.inline_element.as_ref(),
                )
                .key(idx)
                .into(),
                MarkdownElement::Blockquote { content } => rect()
                    .key(idx)
                    .width(Size::fill())
                    .padding(Gaps::new(12., 12., 12., 16.))
                    .border(
                        Border::new()
                            .width(4.)
                            .fill(border_blockquote)
                            .alignment(BorderAlignment::Inner),
                    )
                    .background(background_blockquote)
                    .child(
                        render_content(
                            &content,
                            paragraph_size,
                            color,
                            color_link,
                            color_code,
                            self.inline_element.as_ref(),
                        )
                        .font_slant(FontSlant::Italic),
                    )
                    .into(),
                MarkdownElement::HorizontalRule => rect()
                    .key(idx)
                    .width(Size::fill())
                    .height(Size::px(1.))
                    .background(background_divider)
                    .into(),
                MarkdownElement::Table { headers, rows } => {
                    let mut head = TableHead::new();
                    let mut header_row = TableRow::new();
                    for (col_idx, header_spans) in headers.into_iter().enumerate() {
                        header_row = header_row.child(
                            TableCell::new().key(col_idx).child(
                                render_spans(&header_spans, table_font_size, color, color_code)
                                    .font_weight(FontWeight::BOLD),
                            ),
                        );
                    }
                    head = head.child(header_row);

                    let mut body = TableBody::new();
                    for (row_idx, row) in rows.into_iter().enumerate() {
                        let mut table_row = TableRow::new().key(row_idx);
                        for (col_idx, cell_spans) in row.into_iter().enumerate() {
                            table_row = table_row.child(TableCell::new().key(col_idx).child(
                                render_spans(&cell_spans, table_font_size, color, color_code),
                            ));
                        }
                        body = body.child(table_row);
                    }

                    Table::new().key(idx).child(head).child(body).into()
                }
            };

            container = container.child(child);
        }

        container
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
