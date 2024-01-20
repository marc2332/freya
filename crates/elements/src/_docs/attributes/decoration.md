Specify the decoration in a text.

Accepted values:

- `underline`
- `line-through`
- `overline`

### Example

```rust, no_run
# use freya::prelude::*;
fn app(cx: Scope) -> Element {
    render!(
        label {
            decoration: "line-through",
            "Hello, World!"
        }
    )
}
```
