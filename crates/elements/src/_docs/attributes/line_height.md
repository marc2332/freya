### line_height

Specify the height of the lines of the text.

### Example

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        label {
            line_height: "3",
            "Hello, World! \n Hello, again!"
        }
    )
}
```
