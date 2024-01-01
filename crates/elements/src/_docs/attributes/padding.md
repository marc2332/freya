Specify the inner paddings of an element. You can do so by three different ways, just like in CSS.

### Example

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        rect {
            padding: "25", // 25 in all sides
            padding: "100 50", // 100 in top and bottom, and 50 in left and right
            padding: "5 7 3 9" // 5 in top, 7 in right, 3 in bottom and 9 in left
        }
    )
}
```