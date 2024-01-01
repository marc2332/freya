You can change the alignment of the text using the `text_align` attribute.

Accepted values:

- `center`
- `end`
- `justify`
- `left` (default)
- `right`
- `start`

### Example

```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        label {
            text_align: "right",
            "Hello, World!"
        }
    )
}
```
