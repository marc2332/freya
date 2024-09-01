Determines whether or not elements rendered on a subpixel boundary should be rounded to a physical pixel. Rounding provides better visual clarity (sharp edges/borders on rectangles), but may result in some layouts appearing visually offcenter at small sizes.

If unspecified, rendering will be rounded to physical pixels.

Syntax: `<"round" | "none">`

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            subpixel_rounding: "none"
        }
    )
}
```