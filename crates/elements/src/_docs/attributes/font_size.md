You can specify the size of the text using `font_size`.

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        label {
            font_size: "50",
            "Hellooooo!"
        }
    )
}
```
