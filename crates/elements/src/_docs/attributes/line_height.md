### line_height

Specify the height of the lines of the text.

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            lines_height: "3",
            "Hello, World! \n Hello, again!"
        }
    )
}
```
