trev 🧩
---

A Skia-based desktop renderer for [Dioxus](https://dioxuslabs.com).

```rust

fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!(
        div {
            height: "20%",
            width: "100%",
            background: "black",
            padding: "25",
            p { "Number is: {count}" }
        }
        div {
            height: "40%",
            width: "100%",
            background: "blue",
            padding: "25",
            onclick: move |_| count += 1,
            p { "Increase!" }
        }
    ))
}
```

## To-Do
- [ ] Make padding use SizeMode
- [ ] Add window params to the launch function
- [ ] Support More mouse, keyboard, etc, events.
- [ ] Support for multiple windows
- [ ] Move from dioxus-html and go custom element tags
- [ ] Use [taffy](https://github.com/dioxusLabs/taffy) for Flex layouts.
- [x] Improve the scroll (maybe that needs some kind of layer rendering priorization system? )