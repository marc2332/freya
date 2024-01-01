The `keydown` event fires when the user starts pressing any key.

Event Data: [KeyboardData][crate::events::KeyboardData]

### Example

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        rect {
            onkeydown: |e| println!("Event: {e:?}")
        }
    )
}
```