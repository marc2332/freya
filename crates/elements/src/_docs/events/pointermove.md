The `pointermove` event fires when the user moves the cursor or touches over an element.
Unlike [`onpointerenter`](crate::elements::onpointerenter), this fires even if the user was already hovering over
the element.

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
            onpointermove: |_| println!("Moving or touching!")
        }
    )
}
```