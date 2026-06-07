#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

const MARKDOWN_CONTENT: &str = r#"
# Markdown Viewer Example

This is a demonstration of the **MarkdownViewer** component in Freya.

[![](https://avatars.githubusercontent.com/u/38158676?v=4)]()

[Freya Website](https://freyaui.dev)

## Features

The markdown viewer supports:

- **Bold text** and *italic text*
- ~~Strikethrough~~ text
- `inline code` snippets
- Ordered and unordered lists
- Code blocks with syntax highlighting
- Blockquotes
- Images (with `remote-asset` feature)
- Horizontal rules

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
            .child(MarkdownViewer::new(MARKDOWN_CONTENT).padding(18.)),
    )
}
