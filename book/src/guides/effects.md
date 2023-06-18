# Effects

Learn how the effects attributes work.

- [`rotate`](#rotate)

### rotate

The `rotate` attribute let's you rotate an element.

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            rotate: "180",
            "Hello, World!"
        }
    )
}
```


Compatible elements: all except [`text`](/guides/elements.html#paragraph-and-text).