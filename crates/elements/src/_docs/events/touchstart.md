The `touchstart` event will fire when the user starts touching an element.

Event Data: [TouchData][crate::events::TouchData]

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100",
            height: "100",
            background: "red",
            ontouchstart: |_| println!("Started touching!")
        }
    )
}