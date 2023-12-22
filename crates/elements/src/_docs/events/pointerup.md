The `pointerleave` event will fire when the user stops hovering/touching an element.

Event Data: [PointerData][crate::events::PointerData]

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100",
            height: "100",
            background: "red",
            onpointerleave: |_| println!("Stopped hovering or touching!")
        }
    )
}