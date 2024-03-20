You can choose a style for a text using the `font_style` attribute.

Accepted values:

- `upright` (default)
- `italic`
- `oblique`

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        label {
            font_style: "italic",
            "Hello, italic World!"
        }
    )
}
```

You can also specify multiple fonts in order of priority, if one is not found it will fallback to the next one.

Example: 

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        label {
            font_family: "DoesntExist Font, Impact",
            "Hello, World!"
        }
    )
}
```