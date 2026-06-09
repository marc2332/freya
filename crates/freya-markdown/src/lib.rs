use std::{
    borrow::Cow,
    mem,
};

#[cfg(feature = "remote-asset")]
use freya_components::Uri;
#[cfg(feature = "remote-asset")]
use freya_components::image_viewer::{
    ImageSource,
    ImageViewer,
};
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
use freya_core::prelude::*;
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
use code_editor::render_code_block;

define_theme! {
    %[component]
    pub MarkdownViewer {
        %[fields]
        color: Color,
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
    code_editor_font_family: Cow<'static, str>,
}

impl MarkdownViewer {
    pub fn new(content: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content: content.into(),
            layout: LayoutData::default(),
            key: DiffKey::None,
            theme: None,
            code_editor_font_family: Cow::Borrowed("Jetbrains Mono"),
        }
    }

    /// Sets the font family used for code blocks. Defaults to `"Jetbrains Mono"`.
    pub fn code_editor_font_family(mut self, font_family: impl Into<Cow<'static, str>>) -> Self {
        self.code_editor_font_family = font_family.into();
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
        spans: Vec<TextSpan>,
    },
    CodeBlock {
        code: String,
        language: Option<String>,
    },
    UnorderedList {
        items: Vec<Vec<TextSpan>>,
    },
    OrderedList {
        start: u64,
        items: Vec<Vec<TextSpan>>,
    },
    Image {
        #[cfg_attr(not(feature = "remote-asset"), allow(dead_code))]
        url: String,
        alt: String,
    },
    Link {
        url: String,
        title: Option<String>,
        text: Vec<TextSpan>,
    },
    Blockquote {
        spans: Vec<TextSpan>,
    },
    Table {
        headers: Vec<Vec<TextSpan>>,
        rows: Vec<Vec<Vec<TextSpan>>>,
    },
    HorizontalRule,
}

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
    let mut list_items: Vec<Vec<TextSpan>> = Vec::new();
    let mut current_list_item: Vec<TextSpan> = Vec::new();

    let mut in_heading: Option<HeadingLevel> = None;
    let mut in_paragraph = false;
    let mut in_code_block = false;
    let mut code_block_content = String::new();
    let mut code_block_language: Option<String> = None;
    let mut ordered_list_start: Option<u64> = None;
    let mut in_list_item = false;
    let mut in_blockquote = false;
    let mut blockquote_spans: Vec<TextSpan> = Vec::new();

    let mut in_table_cell = false;
    let mut table_headers: Vec<Vec<TextSpan>> = Vec::new();
    let mut table_rows: Vec<Vec<Vec<TextSpan>>> = Vec::new();
    let mut current_table_row: Vec<Vec<TextSpan>> = Vec::new();
    let mut current_cell_spans: Vec<TextSpan> = Vec::new();

    let mut in_link = false;
    let mut link_url: Option<String> = None;
    let mut link_title: Option<String> = None;
    let mut link_spans: Vec<TextSpan> = Vec::new();

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
                    } else if in_list_item {
                        // Paragraphs inside list items
                    } else {
                        in_paragraph = true;
                        current_spans.clear();
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
                    ordered_list_start = start;
                    list_items.clear();
                }
                Tag::Item => {
                    in_list_item = true;
                    current_list_item.clear();
                }
                Tag::Strong => bold = true,
                Tag::Emphasis => italic = true,
                Tag::Strikethrough => strikethrough = true,
                Tag::BlockQuote(_) => {
                    in_blockquote = true;
                    blockquote_spans.clear();
                }
                Tag::Image {
                    dest_url, title, ..
                } => {
                    elements.push(MarkdownElement::Image {
                        url: dest_url.to_string(),
                        alt: title.to_string(),
                    });
                }
                Tag::Link {
                    dest_url, title, ..
                } => {
                    in_link = true;
                    link_url = Some(dest_url.to_string());
                    link_title = Some(title.to_string());
                    link_spans.clear();
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
                        blockquote_spans.append(&mut current_spans)
                    } else if in_list_item {
                        current_list_item.append(&mut current_spans)
                    } else if in_paragraph {
                        in_paragraph = false;
                        elements.push(MarkdownElement::Paragraph {
                            spans: mem::take(&mut current_spans),
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
                    let items = mem::take(&mut list_items);
                    if let Some(start) = ordered_list_start.take() {
                        elements.push(MarkdownElement::OrderedList { start, items });
                    } else {
                        elements.push(MarkdownElement::UnorderedList { items });
                    }
                }
                TagEnd::Item => {
                    in_list_item = false;
                    list_items.push(mem::take(&mut current_list_item));
                }
                TagEnd::Strong => bold = false,
                TagEnd::Emphasis => italic = false,
                TagEnd::Strikethrough => strikethrough = false,
                TagEnd::BlockQuote(_) => {
                    in_blockquote = false;
                    elements.push(MarkdownElement::Blockquote {
                        spans: mem::take(&mut blockquote_spans),
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
                TagEnd::Link => {
                    in_link = false;
                    if let Some(url) = link_url.take() {
                        elements.push(MarkdownElement::Link {
                            url,
                            title: link_title.take(),
                            text: mem::take(&mut link_spans),
                        });
                    }
                }
                _ => {}
            },
            Event::Text(text) => {
                if in_code_block {
                    code_block_content.push_str(text.trim());
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
                    if in_blockquote && !in_paragraph {
                        blockquote_spans.push(span);
                    } else if in_list_item && !in_paragraph {
                        current_list_item.push(span);
                    } else if in_link {
                        link_spans.push(span);
                    } else {
                        current_spans.push(span);
                    }
                }
            }
            Event::Code(code) => {
                let span = TextSpan {
                    text: code.to_string(),
                    bold,
                    italic,
                    strikethrough,
                    code: true,
                };
                if in_table_cell {
                    current_cell_spans.push(span);
                } else if in_blockquote {
                    blockquote_spans.push(span);
                } else if in_list_item {
                    current_list_item.push(span);
                } else if in_link {
                    link_spans.push(span);
                } else {
                    current_spans.push(span);
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                let span = TextSpan::new(" ");
                if in_blockquote {
                    blockquote_spans.push(span);
                } else if in_list_item {
                    current_list_item.push(span);
                } else if in_link {
                    link_spans.push(span);
                } else {
                    current_spans.push(span);
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

fn render_spans(
    spans: &[TextSpan],
    base_font_size: f32,
    text_color: Color,
    code_color: Color,
) -> Paragraph {
    paragraph()
        .font_size(base_font_size)
        .spans_iter(spans.iter().map(|span| {
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
        }))
}

#[cfg(not(feature = "code-editor"))]
fn render_code_block(
    idx: usize,
    code: String,
    _language: Option<String>,
    background_code: Color,
    color_code: Color,
    code_font_size: f32,
    font_family: Cow<'static, str>,
) -> Element {
    rect()
        .key(idx)
        .width(Size::fill())
        .background(background_code)
        .corner_radius(6.)
        .padding(Gaps::new_all(12.))
        .child(
            label()
                .text(code)
                .font_family(font_family)
                .font_size(code_font_size)
                .color(color_code),
        )
        .into()
}

impl Component for MarkdownViewer {
    fn render(&self) -> impl IntoElement {
        let elements = parse_markdown(&self.content);

        let MarkdownViewerTheme {
            color,
            background_code,
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
                MarkdownElement::Paragraph { spans } => {
                    render_spans(&spans, paragraph_size, color, color_code)
                        .key(idx)
                        .into()
                }
                MarkdownElement::CodeBlock { code, language } => render_code_block(
                    idx,
                    code,
                    language,
                    background_code,
                    color_code,
                    code_font_size,
                    self.code_editor_font_family.clone(),
                ),
                MarkdownElement::UnorderedList { items } => {
                    let mut list = rect()
                        .key(idx)
                        .vertical()
                        .spacing(4.)
                        .padding(Gaps::new(0., 0., 0., 20.));

                    for (item_idx, item_spans) in items.into_iter().enumerate() {
                        let item_content = rect()
                            .key(item_idx)
                            .horizontal()
                            .cross_align(Alignment::Start)
                            .spacing(8.)
                            .child(label().text("•").font_size(paragraph_size).color(color))
                            .child(render_spans(&item_spans, paragraph_size, color, color_code));

                        list = list.child(item_content);
                    }

                    list.into()
                }
                MarkdownElement::OrderedList { start, items } => {
                    let mut list = rect()
                        .key(idx)
                        .vertical()
                        .spacing(4.)
                        .padding(Gaps::new(0., 0., 0., 20.));

                    for (item_idx, item_spans) in items.into_iter().enumerate() {
                        let number = start + item_idx as u64;
                        let item_content = rect()
                            .key(item_idx)
                            .horizontal()
                            .cross_align(Alignment::Start)
                            .spacing(8.)
                            .child(
                                label()
                                    .text(format!("{}.", number))
                                    .font_size(paragraph_size)
                                    .color(color),
                            )
                            .child(render_spans(&item_spans, paragraph_size, color, color_code));

                        list = list.child(item_content);
                    }

                    list.into()
                }
                #[cfg(feature = "remote-asset")]
                MarkdownElement::Image { url, alt } => match url.parse::<Uri>() {
                    Ok(uri) => {
                        let source: ImageSource = uri.into();
                        ImageViewer::new(source)
                            .a11y_alt(alt)
                            .key(idx)
                            .width(Size::fill())
                            .height(Size::px(300.))
                            .into()
                    }
                    Err(_) => label()
                        .key(idx)
                        .text(format!("[Invalid image URL: {}]", url))
                        .color(color)
                        .into(),
                },
                #[cfg(not(feature = "remote-asset"))]
                MarkdownElement::Image { alt, .. } => label()
                    .key(idx)
                    .text(format!("[Image: {}]", alt))
                    .color(color)
                    .into(),
                #[cfg(feature = "router")]
                MarkdownElement::Link { url, title, text } => {
                    let mut tooltip = LinkTooltip::Default;
                    if let Some(title) = title
                        && !title.is_empty()
                    {
                        tooltip = LinkTooltip::Custom(title);
                    }

                    Link::new(url)
                        .tooltip(tooltip)
                        .child(render_spans(&text, paragraph_size, color, color_code))
                        .key(idx)
                        .into()
                }
                #[cfg(not(feature = "router"))]
                MarkdownElement::Link { text, .. } => {
                    render_spans(&text, paragraph_size, color, color_code)
                        .key(idx)
                        .into()
                }
                MarkdownElement::Blockquote { spans } => rect()
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
                        render_spans(&spans, paragraph_size, color, color_code)
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
