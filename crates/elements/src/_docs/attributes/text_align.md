You can change the alignment of the text using the `text_align` attribute.

Accepted values:

- `center`
- `end`
- `justify`
- `left` (default)
- `right`
- `start`

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        label {
            text_align: "right",
            "Hello, World!"
        }
    )
}
```
