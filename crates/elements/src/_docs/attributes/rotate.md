The `rotate` attribute let's you rotate an element.

Compatible elements: all except [`text`](crate::elements::text).

### Example

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        label {
            rotate: "180deg",
            "Hello, World!"
        }
    )
}
```
