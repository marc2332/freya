The `pointerdown` event will fire when the user starts pressing an element.

Event Data: [PointerData][crate::events::PointerData]

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100",
            height: "100",
            background: "red",
            onpointerdown: |_| println!("Started pressing!")
        }
    )
}