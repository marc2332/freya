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
fn app(cx: Scope) -> Element {
    render!(
        label {
            decoration: "line-through",
            decoration_style: "dotted",
            "Hello, World!"
        }
    )
}
```
