### main_align & cross_align

Control how the inner elements are positioned inside the element. You can combine it with the `direction` attribute to create complex flows.

Accepted values for both attributes are:

- `start` (default): At the begining of the axis
- `center`: At the center of the axis
- `end`: At the end of the axis

When using the `vertical` direction, `main_align` will be the Y axis and `cross_align` will be the X axis. But when using the `horizontal` direction, the
`main_align` will be the X axis and the `cross_align` will be the Y axis.

Example on how to center the inner elements in both axis:

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            width: "100%",
            height: "100%",
            main_align: "center",
            cross_align: "center",
            rect {
                width: "50%",
                height: "50%",
                background: "red"
            },
        }
    )
}
```
