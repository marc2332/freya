### min_width & min_height

`rect` supports specifying a minimum width and height, this can be useful if you use it alongside a percentage for the target size.

See syntax for [`Size Units`](crate::_docs::size_unit).

##### Usage

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            background: "red",
            min_width: "100",
            min_height: "100",
            width: "50%",
            height: "50%",
        }
    )
}
```
