You can choose a style for a text using the `font_style` attribute.

Accepted values:

- `upright` (default)
- `italic`
- `oblique`

### Example

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_style: "italic",
            "Hello, italic World!"
        }
    )
}
```
