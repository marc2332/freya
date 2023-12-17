### color

The `color` attribute let's you specify the color of the text.

You can learn about the syntax of this attribute in [`Color Syntax`](/guides/style.html#color-syntax).

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            color: "green",
            "Hello, World!"
        }
    )
}
```

 Another example showing [inheritance](#inheritance):

 ```rust, no_run
 fn app(cx: Scope) -> Element {
     render!(
         rect {
             color: "blue",
             label {
                 "Hello, World!"
             }
         }
     )
 }

 ```