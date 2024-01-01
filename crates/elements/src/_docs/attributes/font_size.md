### font_size

You can specify the size of the text using `font_size`.

Example:

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_size: "50",
            "Hellooooo!"
        }
    )
}
```
