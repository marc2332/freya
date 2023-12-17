The `click` event will fire when the user clicks an element with the left-click.

Event Data: [MouseData][crate::events::MouseData]

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100",
            height: "100",
            background: "red",
            onclick: |_| println!("Clicked!")
        }
    )
}