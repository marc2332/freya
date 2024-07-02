The `mousemove` event fires when the user moves the mouse over an element or any of its children.
Unlike [`mouseenter`](crate::elements::onmouseenter), this fires even if the user was already hovering over
the element.

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
            onmousemove: |_| println!("Hovering!")
        }
    )
}
```