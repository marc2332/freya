# Button

The `Button` component is an styled clicable box. You can place anything inside, like a `label` for example.

Example:

```rust
fn app() -> Element {
    
    let on_button_click = |e: MouseEvent| {
        println!("Button has been clicked!");
    };
    
    render!(
        Button {
            onclick: on_button_click,
            label {
                "hi"
            }
        }
    )
}
```
