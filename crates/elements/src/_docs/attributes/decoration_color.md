Specify the decoration’s color in a text.

You can learn about the syntax of this attribute in [`Color Syntax`](crate::_docs::color_syntax).

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        label {
            decoration: "line-through",
            decoration_color: "orange",
            "Hello, World!"
        }
    )
}
```
