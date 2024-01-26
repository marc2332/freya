The `mouseleave` event fires when the user stops hovering an element.

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
            onmouseleave: |_| println!("Stopped hovering!")
        }
    )
}
```