The `mousedown` event fires when the user starts clicking an element.
Note that this fires for all mouse buttons.
You can check the specific variant with the [`MouseData`](crate::events::MouseData)'s `trigger_button` property.

Event Data: [`MouseData`](crate::events::MouseData)

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            width: "100",
            height: "100",
            background: "red",
            onmousedown: |_| println!("Started clicking!")
        }
    )
}
```