### shadow

Draw a shadow outside of the element.

Syntax: `<x> <y> <intensity> <size> <color>`

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            shadow: "0 0 25 2 rgb(0, 0, 0, 120)"
        }
    )
}
```