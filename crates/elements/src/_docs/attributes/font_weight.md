### font_weight

You can choose a weight for a text using the `font_weight` attribute.

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

Example:

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_weight: "bold",
            "Hello, World!"
        }
    )
}
```
