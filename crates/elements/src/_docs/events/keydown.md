The `keydown` event fires when the user starts pressing any key in the currently focused element.

Event Data: [`KeyboardData`](crate::events::KeyboardData)

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            onkeydown: |e| println!("Event: {e:?}")
        }
    )
}
```