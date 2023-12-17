### font_size

You can specify the size of the text using `font_size`.

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_size: "50",
            "Hellooooo!"
        }
    )
}
```
