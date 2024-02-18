Specify the margin of an element.
You can do so by four different ways, just like in CSS.

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            margin: "25", // 25 in all sides
            margin: "100 50", // 100 in top and bottom, and 50 in left and right
            margin: "2 15 25", // 2 in top, 15 in left and right, and 25 in bottom
            margin: "5 7 3 9" // 5 in top, 7 in right, 3 in bottom and 9 in left
        }
    )
}
```