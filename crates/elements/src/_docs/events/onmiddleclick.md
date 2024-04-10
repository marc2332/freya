The `click` event fires when the user clicks an element with the middle button of the mouse.

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
            onmiddleclick: |_| println!("Clicked!")
        }
    )
}
```