### text_align

You can change the alignment of the text using the `text_align` attribute.

Accepted values: `center`, `end`, `justify`, `left`, `right`, `start`

Example

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            text_align: "right",
            "Hello, World!"
        }
    )
}
```
