Specify how you want the element to be positioned inside it's parent area.

Accepted values:

- `stacked` (default)
- `absolute`

When using the `absolute` mode, you can also combine it with the following attributes:

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
