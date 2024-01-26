# Effects

Learn how the effects attributes work.

- [`rotate`](#rotate)

### rotate

The `rotate` attribute let's you rotate an element.

Example:

```rust, no_run
fn app() -> Element {
    rsx!(
        label {
            rotate: "180deg",
            "Hello, World!"
        }
    )
}
```


Compatible elements: all except [`text`](/guides/elements.html#paragraph-and-text).