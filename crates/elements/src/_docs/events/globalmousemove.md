The `globalmousemove` event fires when the user moves the mouse anywhere in the app.

Event Data: [`MouseData`](crate::events::MouseData)

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            onglobalmousemove: |_| println!("Moving the mouse anywhere!")
        }
        rect {
            width: "100",
            height: "100",
            background: "red",
            onmousemove: |_| println!("Moving the mouse here!")
        }
    )
}
```