Specify the decoration in a text.

Accepted values:

- `underline`
- `line-through`
- `overline`

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        label {
            decoration: "line-through",
            "Hello, World!"
        }
    )
}
```
