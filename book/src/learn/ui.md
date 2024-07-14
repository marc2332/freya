# UI

Freya uses a declarive model for the UI, which means that you do not use imperative APIs but instead you write in a semi-markup language (integrated with Rust) by using the `rsx!()` macro from Dioxus.

For example, this is how a simple component would look like in Freya:

```rs
fn app() -> Element {
    rsx!(
        rect {
            background: "red",
            width: "100%",
            onclick: |_| println!("Clicked!"),
            label {
                "Hello, World!"
            }
        }
    )
}
```

Notice that the `app` component is returning an `Element` created by the `rsx!()` macro. So, in other words, the `Element` contains the UI of that component.
Every time the component reruns the `rsx!()` will be called again and thus generate a new UI.

### `rsx!()`

This macro is not a standalone-language or anything like that. It is simply a macro to easily declare how we want the UI to look like. You can still use normal Rust code inside.

The structure for RSX looks like this:

```rs
rect { // Element
    background: "red", // Attribute for the element `rect`
    width: "100%",// Attribute for the element `rect`
    onclick: |_| println!("Clicked!"), // Event handler for the element `rect`, its just a Rust closure
    label { // Element children of `rect`
        "Hello, World!" // Text Element for the element `label`
    }
}
```

You can reference variables from outside the RSX inside of it:

```rs
let onclick = |_| {
    println!("Clicked");
};

let width = "100%";

rsx!(
    rect {
        background: "red",
        width,
        onclick,
        label {
            "Hello, World!"
        }
    }
)
```

Or just use if, for-loops, etc.. Inside of the RSX:

```rs
let my_value = false;

rsx!(
    rect {
        for i in 0..5 {
            label {
                key: "{i}",
                "Value -> {i}"
            }
        }
        if my_value {
            label {
                "Hello, World!"
            }
        }
    }
)
```