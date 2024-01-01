### font_family

With the `font_family` you can specify what font do you want to use for the inner text.

Limitation: Only fonts installed in the system are supported for now.

Example:

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_family: "Inter",
            "Hello, World!"
        }
    )
}
```
