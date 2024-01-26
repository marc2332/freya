The `globalmouseover` event fires when the user moves the mouse anywhere in the app.

Event Data: [`MouseData`](crate::events::MouseData)

### Example

```rust, no_run
# use freya::prelude::*;
fn app(cx: Scope) -> Element {
    render!(
        rect {
            onglobalmouseover: |_| println!("Moving the mouse somewhere!")
        }
        rect {
            width: "100",
            height: "100",
            background: "red",
            onmousedown: |_| println!("Moving the mouse here!")
        }
    )
}
```