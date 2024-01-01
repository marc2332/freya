The `keyup` event fires when the user releases any key being pressed.

Event Data: [KeyboardData][crate::events::KeyboardData]

### Example

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        rect {
            onkeyup: |e| println!("Event: {e:?}")
        }
    )
}
```