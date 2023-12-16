The `globalmousedown` event will fire when the user starts clicking anywhere with the left-click.

Event Data: [MouseData][crate::events::MouseData]

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            onglobalmousedown: |_| println!("Started clicing somewhere else!")
        }
        rect {
            width: "100",
            height: "100",
            background: "red",
            onmousedown: |_| println!("Started clicking!")
        }
    )
}