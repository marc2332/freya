The `globalclick` event fires when the user clicks anywhere with the left-click.

Event Data: [MouseData][crate::events::MouseData]

### Example:

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        rect {
            onglobalclick: |_| println!("Clicked somewhere else!")
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