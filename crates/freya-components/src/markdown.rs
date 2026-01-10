use std::borrow::Cow;

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

#[cfg(feature = "remote-asset")]
use crate::Uri;
#[cfg(feature = "remote-asset")]
use crate::image_viewer::{
    ImageSource,
    ImageViewer,
};
#[cfg(feature = "router")]
use crate::link::{
    Link,
    LinkTooltip,
};
use crate::{
    table::{
        Table,
        TableBody,
        TableCell,
        TableHead,
        TableRow,
    },
    theming::component_themes::MarkdownViewerTheme,
};

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
    pub(crate) theme: Option<crate::theming::component_themes::MarkdownViewerThemePartial>,
}

impl MarkdownViewer {
    pub fn new(content: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content: content.into(),
            layout: LayoutData::default(),
            key: DiffKey::None,
            theme: None,
        }
    }

    pub fn key(mut self, key: impl Into<DiffKey>) -> Self {
        self.key = key.into();
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

/// Represents different markdown elements for rendering.
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
        #[allow(dead_code)]
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

/// Parse markdown content into a list of elements.
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
                            spans: current_spans.clone(),
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
                            spans: current_spans.clone(),
                        });
                    }
                }
                TagEnd::CodeBlock => {
                    in_code_block = false;
                    elements.push(MarkdownElement::CodeBlock {
                        code: code_block_content.clone(),
                        language: code_block_language.take(),
                    });
                }
                TagEnd::List(_) => {
                    if let Some(start) = ordered_list_start.take() {
                        elements.push(MarkdownElement::OrderedList {
                            start,
                            items: list_items.clone(),
                        });
                    } else {
                        elements.push(MarkdownElement::UnorderedList {
                            items: list_items.clone(),
                        });
                    }
                }
                TagEnd::Item => {
                    in_list_item = false;
                    list_items.push(current_list_item.clone());
                }
                TagEnd::Strong => bold = false,
                TagEnd::Emphasis => italic = false,
                TagEnd::Strikethrough => strikethrough = false,
                TagEnd::BlockQuote(_) => {
                    in_blockquote = false;
                    elements.push(MarkdownElement::Blockquote {
                        spans: blockquote_spans.clone(),
                    });
                }
                TagEnd::Table => {
                    elements.push(MarkdownElement::Table {
                        headers: table_headers.clone(),
                        rows: table_rows.clone(),
                    });
                }
                TagEnd::TableHead => {
                    // TableHead contains cells directly (no TableRow), so save headers here
                    table_headers = current_table_row.clone();
                    current_table_row.clear();
                }
                TagEnd::TableRow => {
                    // TableRow only appears in body rows, not in TableHead
                    table_rows.push(current_table_row.clone());
                    current_table_row.clear();
                }
                TagEnd::TableCell => {
                    in_table_cell = false;
                    current_table_row.push(current_cell_spans.clone());
                }
                TagEnd::Link => {
                    in_link = false;
                    if let Some(url) = link_url.take() {
                        elements.push(MarkdownElement::Link {
                            url,
                            title: link_title.take(),
                            text: link_spans.clone(),
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

/// Render text spans as a paragraph element.
fn render_spans(spans: &[TextSpan], base_font_size: f32, code_color: Option<Color>) -> Paragraph {
    let mut p = paragraph().font_size(base_font_size);

    for span in spans {
        let mut s = Span::new(span.text.clone());

        if span.bold {
            s = s.font_weight(FontWeight::BOLD);
        }

        if span.italic {
            s = s.font_slant(FontSlant::Italic);
        }

        if span.code {
            s = s.font_family("monospace");
            if let Some(c) = code_color {
                s = s.color(c);
            }
        }

        p = p.span(s);
    }

    p
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
        } = crate::get_theme!(&self.theme, markdown_viewer);

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
                    render_spans(&spans, font_size, Some(color))
                        .font_weight(FontWeight::BOLD)
                        .key(idx)
                        .into()
                }
                MarkdownElement::Paragraph { spans } => {
                    render_spans(&spans, paragraph_size, Some(color))
                        .key(idx)
                        .into()
                }
                MarkdownElement::CodeBlock { code, .. } => rect()
                    .key(idx)
                    .width(Size::fill())
                    .background(background_code)
                    .corner_radius(6.)
                    .padding(Gaps::new_all(12.))
                    .child(
                        label()
                            .text(code)
                            .font_family("monospace")
                            .font_size(code_font_size)
                            .color(color_code),
                    )
                    .into(),
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
                            .child(label().text("•").font_size(paragraph_size))
                            .child(render_spans(&item_spans, paragraph_size, Some(color_code)));

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
                                    .font_size(paragraph_size),
                            )
                            .child(render_spans(&item_spans, paragraph_size, Some(color_code)));

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
                        .into(),
                },
                #[cfg(not(feature = "remote-asset"))]
                MarkdownElement::Image { alt, .. } => {
                    label().key(idx).text(format!("[Image: {}]", alt)).into()
                }
                MarkdownElement::Link { url, title, text } => {
                    #[cfg(feature = "router")]
                    {
                        let mut tooltip = LinkTooltip::Default;
                        if let Some(title) = title {
                            if !title.is_empty() {
                                tooltip = LinkTooltip::Custom(title);
                            }
                        }

                        Link::new(url)
                            .tooltip(tooltip)
                            .child(render_spans(&text, paragraph_size, Some(color)))
                            .key(idx)
                            .into()
                    }
                    #[cfg(not(feature = "router"))]
                    {
                        render_spans(&text, paragraph_size, Some(color))
                            .key(idx)
                            .into()
                    }
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
                        render_spans(&spans, paragraph_size, Some(color_code))
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
                    let columns = headers.len();

                    let mut head = TableHead::new();
                    let mut header_row = TableRow::new();
                    for (col_idx, header_spans) in headers.into_iter().enumerate() {
                        header_row = header_row.child(
                            TableCell::new().key(col_idx).child(
                                render_spans(&header_spans, table_font_size, Some(color_code))
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
                                render_spans(&cell_spans, table_font_size, Some(color_code)),
                            ));
                        }
                        body = body.child(table_row);
                    }

                    Table::new(columns).key(idx).child(head).child(body).into()
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
