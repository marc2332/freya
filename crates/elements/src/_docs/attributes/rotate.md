 ### rotate

 The `rotate` attribute let's you rotate an element.

 Example:

 ```rust, no_run
# use dioxus::prelude::*;
# use freya_elements::elements as dioxus_elements;
fn app(cx: Scope) -> Element {
    render!(
        label {
            rotate: "180deg",
            "Hello, World!"
        }
    )
}
```

Compatible elements: all except [`text`](/guides/elements.html#paragraph-and-text).
