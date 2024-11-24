The `keyup` event fires when the user releases any key being pressed in the currently focused element.

Event Data: [`KeyboardData`](crate::events::KeyboardData)

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            onkeyup: |e| println!("Event: {e:?}")
        }
    )
}
```