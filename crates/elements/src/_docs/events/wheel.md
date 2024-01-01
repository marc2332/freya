The `wheel` event fires when the user scrolls the mouse wheel while hovering over the element.

Event Data: [`TouchData`](crate::events::TouchData)

### Example

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100",
            height: "100",
            background: "red",
            onwheel: |_| println!("Scrolling with the wheel!")
        }
    )
}
```