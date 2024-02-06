With the `font_family` you can specify what font you want to use for the inner text.

Check out the [custom font example](https://github.com/marc2332/freya/blob/main/examples/custom_font.rs)
to see how you can load your own fonts.

<!-- TODO: Example of checking if a font exists with skia_safe -->

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        label {
            font_family: "Inter",
            "Hello, World!"
        }
    )
}
```
