use freya::prelude::*;

const CONTENT: &str = r#"
# Freya on Android

This is a **MarkdownViewer** running on Android.

## Features

- **Bold text** and *italic text*
- ~~Strikethrough~~ text
- `inline code` snippets

## Code Example

```rust
fn main() {
    println!("Hello from Freya!");
}
```

## Lists

1. First step
2. Second step
3. Third step

> Freya makes it easy to build cross-platform UIs with Rust.

| Component | Status |
|-----------|--------|
| Switch | Done |
| Slider | Done |
| Select | Done |
"#;

#[derive(PartialEq)]
pub struct MarkdownDemo;

impl Component for MarkdownDemo {
    fn render(&self) -> impl IntoElement {
        rect().expanded().child(
            ScrollView::new()
                .width(Size::fill())
                .height(Size::fill())
                .child(MarkdownViewer::new(CONTENT).padding(16.)),
        )
    }
}
