Specify the spacing between characters of the text.

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        label {
            letter_spacing: "10",
            "Hello, World!"
        }
    )
}
```
