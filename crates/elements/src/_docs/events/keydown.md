The `keydown` event will fire when the user starts pressing any key.

Event Data: [KeyboardData][crate::events::KeyboardData]

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            keydown: |e| println!("Event: {e:?}")
        }
    )
}