### text_overflow

Determines how text is treated when it exceeds its [`max_lines`](#max_lines) count. By default uses the `clip` mode, which will cut off any overflowing text, with `ellipsis` mode it will show `...` at the end.

Accepted values:

- `clip` (default)
- `ellipsis`

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            max_lines: "3",
            text_overflow: "ellipsis",
            "Looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong text"
        }
    )
}
```
