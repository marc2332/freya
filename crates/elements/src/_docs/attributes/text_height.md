Specify the text height behavior.

Accepted values:

- `disable-all` (default)
- `all`
- `disable-first-ascent`
- `disable-least-ascent`

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        label {
            text_height: "disable-all",
            "Hello, World!"
        }
    )
}
```