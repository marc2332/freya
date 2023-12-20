The `keyup` event will fire when the user releases any key being pressed.

Event Data: [KeyboardData][crate::events::KeyboardData]

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            keyup: |e| println!("Event: {e:?}")
        }
    )
}