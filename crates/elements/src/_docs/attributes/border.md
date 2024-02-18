### border & border_align

You can add a border to an element using the `border` and `border_align` attributes.
- `border` syntax: `[width] <solid | none> [color]`.
- `border_align` syntax: `<inner | outer | center>`.

### Example
```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            border: "2 solid black",
            border_align: "inner"
        }
    )
}
```