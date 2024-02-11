Specify the spacing between words of the text.

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        label {
            word_spacing: "10",
            "Hello, World!"
        }
    )
}
```
