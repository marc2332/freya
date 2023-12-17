### decoration_color

Specify the decoration's color in a text.

You can learn about the syntax of this attribute in [`Color Syntax`](/guides/style.html#color-syntax).

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            decoration: "line-through",
            decoration_color: "orange",
            "Hello, World!"
        }
    )
}
```
