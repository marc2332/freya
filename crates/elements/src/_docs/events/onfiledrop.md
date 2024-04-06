The `filedrop` event fires when the user drops a file over the element.

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
            onfiledrop: |e| println!("File dropped: {e:?}")
        }
    )
}
```