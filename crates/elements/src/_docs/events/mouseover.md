The `mouseover` event fires when the user starts hovering an element. Usually used in combination of [`mouseleave`](crate::elements::onmouseleave).

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
            onmouseover: |_| println!("Started hovering!")
        }
    )
}
```