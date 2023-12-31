### decoration_style

Specify the decoration's style in a text.

Accpted values:

- `solid` (default)
- `double`
- `dotted`
- `dashed`
- `wavy`

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            decoration: "line-through",
            decoration_style: "dotted",
            "Hello, World!"
        }
    )
}
```
