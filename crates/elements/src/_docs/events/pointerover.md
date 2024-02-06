The `pointerover` event fires when the user hovers/touches over an element.
Unlike [`onpointerenter`](crate::elements::onpointerenter), this fires even if the user was already hovering over
the element. For that reason, it's less efficient.

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
            onpointerover: |_| println!("Hovering or touching!")
        }
    )
}
```