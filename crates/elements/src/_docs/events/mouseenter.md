The `mouseenter` event fires when the user starts hovering an element.

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
            onmouseenter: |_| println!("Started hovering!")
        }
    )
}
```