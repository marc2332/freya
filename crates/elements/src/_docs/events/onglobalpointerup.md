The `globalpointerup` event fires when the user releases the point anywhere in the app.

Event Data: [`PointerData`](crate::events::PointerData)

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            onglobalpointerup: |_| println!("Pointer released somewhere else!")
        }
        rect {
            width: "100",
            height: "100",
            background: "red",
            onclick: |_| println!("Clicked!")
        }
    )
}
```