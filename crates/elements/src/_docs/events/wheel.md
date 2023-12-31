The `wheel` event will fire when the user scrolls the mouse wheel.

Event Data: [TouchData][crate::events::TouchData]

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100",
            height: "100",
            background: "red",
            onwheel: |_| println!("Scrolling with the wheel!")
        }
    )
}