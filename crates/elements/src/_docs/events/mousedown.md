The `mousedown` event will fire when the user starts clicking an element.

Event Data: [MouseData][crate::events::MouseData]

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100",
            height: "100",
            background: "red",
            onmousedown: |_| println!("Started clicking!")
        }
    )
}