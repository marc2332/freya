The `color` attribute lets you specify the color of the text.

You can learn about the syntax of this attribute in [`Color Syntax`](crate::_docs::color_syntax).

### Example

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        label {
            color: "green",
            "Hello, World!"
        }
    )
}
```

Another example showing [inheritance](crate::_docs::inheritance):

```rust, no_run
# use freya::prelude::*;
fn app() -> Element {
    rsx!(
        rect {
            color: "blue",
            label {
                "Hello, World!"
            }
        }
    )
}
```