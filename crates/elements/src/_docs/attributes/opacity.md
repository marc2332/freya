Specify the opacity of an element and all its descendants.

### Example

```rust, no_run
# use freya::prelude::*;
fn app(cx: Scope) -> Element {
    render!(
        rect {
            opacity: "0.5", // 50% visible
            label {
                "I am fading!"
            }
        }
    )
}
```