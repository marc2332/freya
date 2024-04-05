The `onglobalfilehovercancelled` event fires when the user cancels the hovering of a file over the window. It's the opposite of [`onglobalfilehover`](crate::elements::onglobalfilehover).

Event Data: [`FileData`](crate::events::FileData)

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            width: "100%",
            height: "100%",
            background: "black",
            onglobalfilehovercancelled: |e| println!("File hover cancelled: {e:?}")
        }
    )
}
```
