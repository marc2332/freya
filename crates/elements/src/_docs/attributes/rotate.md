The `rotate` attribute let's you rotate an element.

Compatible elements: all except [`text`](crate::elements::text).

### Example

```rust, no_run
# use freya::prelude::*;
fn app(cx: Scope) -> Element {
    render!(
        label {
            rotate: "180deg",
            "Hello, World!"
        }
    )
}
```
