The `mouseup` event fires when the user ends the click in an element with the left button of the mouse.

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
            onmouseup: |_| println!("Clicked!")
        }
    )
}
```