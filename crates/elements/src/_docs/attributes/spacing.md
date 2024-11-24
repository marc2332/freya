Specify a space between the inner elements. Think it as a margin for every element but defined by its parent.
It only applies to the side of the direction.

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            direction: "vertical",
            spacing: "20",
            // Not before
            rect {
                width: "100",
                height: "100",
                background: "red",
            }
            // There will be a space between these two elements of 20 pixels
            rect {
                width: "100",
                height: "100",
                background: "blue",
            }
            // Here as well
            rect {
                width: "100",
                height: "100",
                background: "green",
            }
            // But not after
        }
    )
}
```