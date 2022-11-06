# container

`container` provides a box with a certain `width` and `height`. 

It's children are clipped when they overflow the container bounds.

### Supported attribute
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
        container {
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