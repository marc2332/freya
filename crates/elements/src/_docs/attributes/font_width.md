### font_width

You can choose a width for a text using the `font_width` attribute.

Accepted values:

- `ultra-condensed`
- `extra-condensed`
- `condensed`
- `normal` (default)
- `semi-expanded`
- `expanded`
- `extra-expanded`
- `ultra-expanded`

Example:

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_weight: "bold",
            "Hello, World!"
        }
    )
}
```
