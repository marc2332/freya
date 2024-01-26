Specify the width and height for the given element.

See syntax in [`Size Units`](crate::_docs::size_unit).

### Example

```rust, no_run
# use freya::prelude::*;
fn app(cx: Scope) -> Element {
    render!(
        rect {
            background: "red",
            width: "15",
            height: "50",
        }
    )
}
```