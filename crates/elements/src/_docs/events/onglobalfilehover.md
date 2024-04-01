The `onglobalfilehover` event fires when the user hovers a file over the window.

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
            onglobalfilehover: |e| println!("File hover: {e:?}")
        }
    )
}
```