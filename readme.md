trev 🧩
---

A Skia-based desktop renderer for [Dioxus](https://dioxuslabs.com).

```rust

fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);

    cx.render(rsx!(
        view {
            height: "20%",
            width: "100%",
            background: "black",
            padding: "25",
            p { tabindex: "1", "Number is: {count}" }
        }
        view {
            height: "80%",
            width: "100%",
            background: "blue",
            padding: "25",
            onclick: move |_| count += 1,
            p { tabindex: "1", "Increase!" }
        }
    ))
}
```

## To-Do
- [ ] Make padding use SizeMode
- [ ] Add window params to the launch function
- [ ] Improve and support more mouse, keyboard, etc, events.
- [ ] Support for multiple windows
- [ ] Move from dioxus-html and go custom element tags
- [ ] Use [taffy](https://github.com/dioxusLabs/taffy) for Flex layouts.
- [x] Priorize inner scroll views inside scroll views.