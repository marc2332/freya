### decoration_style

Specify the decoration's style in a text.

Accepted values:

- `solid` (default)
- `double`
- `dotted`
- `dashed`
- `wavy`

Example:

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        label {
            decoration: "line-through",
            decoration_style: "dotted",
            "Hello, World!"
        }
    )
}
```
