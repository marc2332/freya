# Built-in Hooks

Freya comes with a set hooks to simplify various tasks, such as animations, accessibility, text editing and more.

You can find more about them in [their docs](https://docs.rs/freya-hooks). 

Example:
```rs
fn app() -> Element {
    let mut my_focus = use_focus();

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            focus_id: my_focus.attribute(),
            onclick: move |_| my_focus.focus(),
            label {
                "{my_focus.is_focused()}"
            }
        }
    )
}
```