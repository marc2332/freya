### corner_radius & corner_smoothing

The `corner_radius` attribute lets you smooth the corners of the element, with `corner_smoothing` you can give a "squircle" effect.

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            corner_radius: "10",
            corner_smoothing: "75%"
        }
    )
}
```