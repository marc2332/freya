### main_align & cross_align

Control how the inner elements are positioned inside the element. You can combine it with the `direction` attribute to create complex flows.

Accepted values for `main_align`:

- `start` (default): At the begining of the axis
- `center`: At the center of the axis
- `end`: At the end of the axis
- `space-between`(only for `main_align`): Distributed among the available space
- `space-around` (only for `main_align`): Distributed among the available space with small margins in the sides
- `space-evenly` (only for `main_align`): Distributed among the available space with the same size of margins in the sides and in between the elements.

Accepted values for `cross_align`:

- `start` (default): At the begining of the axis (same as in `main_align`)
- `center`: At the center of the axis (same as in `main_align`)
- `end`: At the end of the axis (same as in `main_align`)

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
