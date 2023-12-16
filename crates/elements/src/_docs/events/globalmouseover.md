The `globalmouseover` event will fire when the user moves the mouse cursor anywhere in the app.

Event Data: [MouseData][crate::events::MouseData]

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            onglobalmouseover: |_| println!("Moving somewhere else!")
        }
        rect {
            width: "100",
            height: "100",
            background: "red",
            onmouseover: |_| println!("Moving!")
        }
    )
}