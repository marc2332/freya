Specify the decoration's style in a text.

Accepted values:

- `solid` (default)
- `double`
- `dotted`
- `dashed`
- `wavy`

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        label {
            decoration: "line-through",
            decoration_style: "dotted",
            "Hello, World!"
        }
    )
}
```
