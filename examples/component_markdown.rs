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

This is a demonstration of the **MarkdownViewer** component in Freya. <badge/>

Custom inline elements like this counter <counter/> can flow within the text.

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
fn main() {
    println!("Hello, Freya!");
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
                    .padding(18.)
                    .inline_element(|html| {
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
