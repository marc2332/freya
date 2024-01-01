The `pointerup` event fires when the user releases their mouse button or stops touching the element.

Event Data: [`PointerData`](crate::events::PointerData)

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
            onpointerup: |_| println!("Released mouse button, or no longer touching!")
        }
    )
}
```