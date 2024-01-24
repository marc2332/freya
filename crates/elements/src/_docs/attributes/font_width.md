You can choose a width for a text using the `font_width` attribute.

⚠️ Only fonts with variable widths will be affected.

Accepted values:

- `ultra-condensed`
- `extra-condensed`
- `condensed`
- `normal` (default)
- `semi-expanded`
- `expanded`
- `extra-expanded`
- `ultra-expanded`

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        label {
            font_width: "ultra-expanded",
            "Hello, wide World!"
        }
    )
}
```
