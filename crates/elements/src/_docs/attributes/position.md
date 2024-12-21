Specify how you want the element to be positioned inside it's parent area.

Accepted values:

- `stacked` (default)
- `absolute` (Floating element relative to the parent element)
- `global` (Floating element relative to the window)

When using the `absolute` or `global` modes, you can also combine them with the following attributes:

- `position_top`
- `position_right`
- `position_bottom`
- `position_left`

These only support pixels.

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            width: "100%",
            height: "100%",
            rect {
                position: "absolute",
                position_bottom: "15",
                position_right: "15",
                background: "black",
                width: "100",
                height: "100",
            }
        }
    )
}
```
