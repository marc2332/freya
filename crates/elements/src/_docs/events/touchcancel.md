The `touchcancel` event fires when the user cancels the touching, this is usually caused by the hardware or the OS.
Also see [`ontouchend`](crate::elements::ontouchend).

Event Data: [`TouchData`](crate::events::TouchData)

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            width: "100",
            height: "100",
            background: "red",
            ontouchcancel: |_| println!("Touching canceled!")
        }
    )
}
```