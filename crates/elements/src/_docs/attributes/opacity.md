### opacity

Specify the opacity of an element and all its desdendants.

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            opacity: "0.5", // 50% visible
            label {
                "I am fading!"
            }
        }
    )
}
```