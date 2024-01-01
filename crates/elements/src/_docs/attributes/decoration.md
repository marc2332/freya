Specify the decoration in a text.

Accepted values:

- `underline`
- `line-through`
- `overline`

### Example

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        label {
            decoration: "line-through",
            "Hello, World!"
        }
    )
}
```
