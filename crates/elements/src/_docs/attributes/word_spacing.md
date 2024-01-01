### word_spacing

Specify the spacing between words of the text.

Example:

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        label {
            word_spacing: "10",
            "Hello, World!"
        }
    )
}
```
