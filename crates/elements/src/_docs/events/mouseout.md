The `mouseout` event fires when the user stops hovering an element. Used in combination of [`mouseover`](crate::elements::mouseover).

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
            onmouseout: |_| println!("Stopped hovering!")
        }
    )
}
```