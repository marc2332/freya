You can choose a weight for text using the `font_weight` attribute.

Accepted values:

- `invisible`
- `thin`
- `extra-light`
- `light`
- `normal` (default)
- `medium`
- `semi-bold`
- `bold`
- `extra-bold`
- `black`
- `extra-black`
- `50`
- `100`
- `200`
- `300`
- `400`
- `500`
- `600`
- `700`
- `800`
- `900`
- `950`

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        label {
            font_weight: "bold",
            "Hello, bold World!"
        }
    )
}
```
