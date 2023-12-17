### letter_spacing

Specify the spacing between characters of the text.

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            letter_spacing: "10",
            "Hello, World!"
        }
    )
}
```
