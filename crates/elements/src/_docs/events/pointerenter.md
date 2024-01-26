The `pointerenter` event fires when the user starts hovering/touching an element.

Event Data: [`PointerData`](crate::events::PointerData)

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            width: "100",
            height: "100",
            background: "red",
            onpointerenter: |_| println!("Started hovering or touching!")
        }
    )
}
```