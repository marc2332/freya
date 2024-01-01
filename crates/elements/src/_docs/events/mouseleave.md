The `mouseleave` event fires when the user stops hovering an element.

Event Data: [MouseData][crate::events::MouseData]

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
            onmouseleave: |_| println!("Stopped hovering!")
        }
    )
}
```