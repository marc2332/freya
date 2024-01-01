The `touchcancel` event fires when the user cancels the touching, this is usually caused by the hardware or the OS.

Event Data: [TouchData][crate::events::TouchData]

### Example:

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100",
            height: "100",
            background: "red",
            ontouchcancel: |_| println!("Touching canceled!")
        }
    )
}
```