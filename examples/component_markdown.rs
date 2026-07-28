#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    code_editor::EditorLanguage,
    prelude::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn resolve_language(language: String) -> Option<EditorLanguage> {
    match language.as_str() {
        "rust" => Some(EditorLanguage::new(
            tree_sitter_rust::LANGUAGE,
            tree_sitter_rust::HIGHLIGHTS_QUERY,
        )),
        _ => None,
    }
}

const MARKDOWN_CONTENT: &str = r#"
# Markdown Viewer Example

This is a demonstration of the **MarkdownViewer** component in Freya. <badge/>

Custom inline elements like this counter <counter/> can flow within the text.

[![](https://avatars.githubusercontent.com/u/38158676?v=4)]()

[Freya Website](https://freyaui.dev)

## Features

The markdown viewer supports:

- **Bold text** and *italic text*
- Ordered and unordered lists
- Code blocks with syntax highlighting
- Blockquotes
- Images (with `remote-asset` feature)
- Horizontal rules
- And more!

---

## Code Example

Here's a Rust code block:

```rust
fn app() -> impl IntoElement {
    let mut count = use_state(|| 4);

    let counter = rect()
        .width(Size::fill())
        .height(Size::percent(50.))
        .center()
        .color((255, 255, 255))
        .background((15, 163, 242))
        .font_weight(FontWeight::BOLD)
        .font_size(75.)
        .shadow((0., 4., 20., 4., (0, 0, 0, 80)))
        .child(count.read().to_string());

    let actions = rect()
        .horizontal()
        .width(Size::fill())
        .height(Size::percent(50.))
        .center()
        .spacing(8.0)
        .child(
            Button::new()
                .on_press(move |_| {
                    *count.write() += 1;
                })
                .child("Increase"),
        )
        .child(
            Button::new()
                .on_press(move |_| {
                    *count.write() -= 1;
                })
                .child("Decrease"),
        );

    rect().child(counter).child(actions)
}
```

## Lists

### Unordered List

- First item
- Second item with **bold**
- Third item with `code`

### Ordered List

1. First step
2. Second step
3. Third step

## Tables

| Name | Age | City |
|------|-----|------|
| Alice | 30 | New York |
| Bob | 25 | San Francisco |
| Charlie | 35 | London |

## Blockquote

> This is a blockquote.
> It can span multiple lines.

## Conclusion

The markdown viewer makes it easy to render rich text content in your Freya applications!
"#;

fn app() -> impl IntoElement {
    rect().expanded().child(
        ScrollView::new()
            .width(Size::fill())
            .height(Size::fill())
            .child(
                MarkdownViewer::new(MARKDOWN_CONTENT)
                    .code_editor_language(resolve_language)
                    .padding(18.)
                    .inline_element(|html: String| {
                        if html.starts_with("<counter") {
                            Some(Counter.into_element())
                        } else if html.starts_with("<badge") {
                            Some(
                                rect()
                                    .background((0, 119, 182))
                                    .corner_radius(8.)
                                    .padding(Gaps::new(2., 8., 2., 8.))
                                    .child(label().text("New").color(Color::WHITE).font_size(12.))
                                    .into_element(),
                            )
                        } else {
                            None
                        }
                    }),
            ),
    )
}

#[derive(PartialEq)]
struct Counter;

impl Component for Counter {
    fn render(&self) -> impl IntoElement {
        let mut count = use_state(|| 0);
        Button::new()
            .rounded_full()
            .on_press(move |_| *count.write() += 1)
            .child(format!("Clicked {} times", count.read()))
    }
}
