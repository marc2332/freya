The `pointerleave` event fires when the user stops hovering/touching an element.

Event Data: [`PointerData`](crate::events::PointerData)

### Example

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100",
            height: "100",
            background: "red",
            onpointerleave: |_| println!("Started hovering or touching!")
        }
    )
}
```