### word_spacing

Specify the spacing between words of the text.

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            word_spacing: "10",
            "Hello, World!"
        }
    )
}
```
