### decoration

Specify the decoration in a text.

Accpted values:

- `underline`
- `line-through`
- `overline`

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            decoration: "line-through",
            "Hello, World!"
        }
    )
}
```
