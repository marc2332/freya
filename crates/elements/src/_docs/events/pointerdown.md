The `pointerdown` event fires when the user clicks/starts touching an element.

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
            onpointerdown: |_| println!("Clicked/started pressing!")
        }
    )
}
```