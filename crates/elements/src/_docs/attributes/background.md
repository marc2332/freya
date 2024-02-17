Specify a color as the background of an element.

You can learn about the syntax of this attribute [here](#color-syntax).

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            background: "red"
        }
    )
}
```