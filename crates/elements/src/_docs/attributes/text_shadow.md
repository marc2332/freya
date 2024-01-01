### text_shadow

Specify the shadow of a text.

Syntax: `<x> <y> <size> <color>`

Example:

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        label {
            text_shadow: "0 18 12 rgb(0, 0, 0)",
            "Hello, World!"
        }
    )
}
```
