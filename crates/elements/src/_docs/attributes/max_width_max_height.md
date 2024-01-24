### max_width & max_height

`rect` supports specifying a maximum width and height.

See syntax for [`Size Units`](crate::_docs::size_unit).

##### Usage

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            background: "red",
            max_width: "50%",
            max_height: "50%",
            width: "500",
            height: "500",
        }
    )
}
```
