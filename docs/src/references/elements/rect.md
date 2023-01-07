# rect

`rect` is a box and unlike [`container`](/references/elements/container.html), it's children can overflow as much as they want.

### Supported attributes
- width
- height
- min_width
- min_height
- max_width
- max_height
- background
- padding
- layer
- scroll_x
- scroll_y
- direction
- shadow
- radius
- color
- display

### Usage

```rust
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100%",
            height: "100%",
            padding: "25",
            background: "yellow",
            label {
                color: "black",
                "Hello World :)"
            }
        }
    )
}
```