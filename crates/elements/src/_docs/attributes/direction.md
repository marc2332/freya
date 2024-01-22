Control how the inner elements stack.

Accepted values:

- `vertical` (default)
- `horizontal`

##### Usage

```rust, no_run
# use freya::prelude::*;
fn app(cx: Scope) -> Element {
    rsx!(
        rect {
            width: "100%",
            height: "100%",
            direction: "vertical",
            rect {
                width: "100%",
                height: "50%",
                background: "red"
            },
            rect {
                width: "100%",
                height: "50%",
                background: "green"
            }
        }
    )
}
```
