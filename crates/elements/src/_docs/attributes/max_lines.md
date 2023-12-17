### max_lines

Determines the amount of lines that the text can have. It has unlimited lines by default.

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            "Hello, World! \n Hello, World! \n Hello, world!" // Will show all three lines
        }
        label {
            lines_height: "2",
            "Hello, World! \n Hello, World! \n Hello, world!" // Will only show two lines
        }
    )
}
```
