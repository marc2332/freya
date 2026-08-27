use freya::prelude::*;

const CONTENT: &str = r#"
# Trip notes

Written in a text file, shown as a page.

## What it covers

- **Bold** and *italic* text
- `inline code`
- Lists, tables and quotes

```rust
fn main() {
    println!("Hello there");
}
```

> Long quotes get their own little margin.

| Feature | Status |
| --- | --- |
| Headings | Done |
| Tables | Done |
| Code blocks | Done |
"#;

#[derive(PartialEq)]
pub struct MarkdownShowcase;

impl Component for MarkdownShowcase {
    fn render(&self) -> impl IntoElement {
        ScrollView::new().child(
            rect()
                .spacing(20.)
                .child(super::heading("Markdown", "Some markdown"))
                .child(MarkdownViewer::new(CONTENT.to_string())),
        )
    }
}
