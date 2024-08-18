# Built-in Components

Freya comes with a set of styled and functional components you may use to develop faster. Some examples as `Button`, `Switch`, `Scrollview`, etc.

You can find more about them in [their docs](https://docs.rs/freya-components). 

Example:
```rs
fn app() -> Element {
    let mut enabled = use_signal(|| true);

    rsx!(
        ScrollView {
            Button {
                onclick: |_| {
                    println!("Button was clicked!");
                }
            }
            Switch {
                enabled: enabled(),
                ontoggled: move |_| {
                    enabled.toggle();
                }
            }
        } 
    )
}
```