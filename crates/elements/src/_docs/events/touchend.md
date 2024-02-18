The `touchend` event fires when the user stops touching an element.

Event Data: [`TouchData`](crate::events::TouchData)

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            width: "100",
            height: "100",
            background: "red",
            ontouchend: |_| println!("Stopped touching!")
        }
    )
}
```