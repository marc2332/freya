# trev 🧩

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
            text { "Number is: {count}" }
        }
        view {
            height: "80%",
            width: "100%",
            background: "blue",
            padding: "25",
            onclick: move |_| count += 1,
            text { "Increase!" }
        }
    ))
}
```

## Concepts / Ideas
### Headless running
Ability to run the app without actually rendering, this is useful for testing.

### Elements and components
The idea is so have primitive elements such as text and view, and then have components that can be used to build more complex elements such scroll views, buttons, input fields, etc.

### Tauri integration
Inspired by [tauri-egui](https://github.com/tauri-apps/tauri-egui).

## To-Do
- [ ] Make padding use SizeMode
- [ ] Add window params to the launch function
- [ ] Improve and support more mouse, keyboard, etc, events.
- [ ] Support for multiple windows
- [x] Move from dioxus-html and go custom element tags
- [ ] Use [taffy](https://github.com/dioxusLabs/taffy) for Flex layouts.
- [x] Priorize inner scroll views inside scroll views.


MIT License