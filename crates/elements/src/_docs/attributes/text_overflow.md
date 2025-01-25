Determines how text is treated when it exceeds its [`max_lines`](#max_lines) count. By default uses the `clip` mode, which will cut off any overflowing text, with `ellipsis` mode it will show `...` at the end.

Accepted values:

- `clip` (default): Simply cut the text.
- `ellipsis`: Show `…`.
- `<custom-value>: Show a custom value.

### Ellipsis example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        label {
            max_lines: "3",
            text_overflow: "ellipsis",
            "Looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong text"
        }
    )
}
```

### Custom value example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        label {
            max_lines: "3",
            text_overflow: ".......too long.",
            "Looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong text"
        }
    )
}
```

