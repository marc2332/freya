Specify the inner paddings of an element. You can do so by four different ways, just like in CSS.

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            padding: "25", // 25 in all sides
            padding: "100 50", // 100 in top and bottom, and 50 in left and right
            padding: "2 15 25", // 2 in top, 15 in left and right, and 25 in bottom
            padding: "5 7 3 9" // 5 in top, 7 in right, 3 in bottom and 9 in left
        }
    )
}
```