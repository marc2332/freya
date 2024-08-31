# Components

Freya apps will usually be composed of different components.
Components are defined in the form functions that might receive some input as **Props** and return the UI as **Element**.

This is how a simple root component looks like:

```rs
fn app() -> Element {
    rsx!(
        label {
            "Hello, World!"
        }
    )
}
```

This is obviously fine, but the moment our app grows in size and complexity we might want to split
things out in order to maintain a certain level of modularity and reusability. We can do this with components.

For example:

This is how a simple root component looks like:

```rs
// Root component that gets passed to the `launch` function
fn app() -> Element {
    rsx!(
        TextLabel {
            "Number 1"
        }
        label {
            "Number 2"
        }
        TextLabel {
            "Number 3"
        }
    )
}

// Reusable component
#[component]
fn TextLabel(text: String) -> Element {
    rsx!(
        label {
            "{text}"
        }
    )
}
```

Notice how we anotate our `TextLabel` component with the macro `#[component]`, this will transform every argument of the function (just `text: String` in this case) to a component prop, so we can later use the component in a declarative way in the RSX.

For more complex components you might want to put the props in an external struct intead of using the `#[components]` macro:

```rs
#[derive(Props, PartialEq, Clone)]
struct TextLabelProps {
    text: String
}

fn TextLabel(TextLabelProps { text }: TextLabelProps) -> Element {
    rsx!(
        label {
            "{text}"
        }
    )
}
```

## Renders

Components renders are just when a component function runs, which might be because it is subscribed to a signal and that signal got mutated. Consider this simple component:

```rs
#[component]
fn CoolComp() -> Element {
    let mut count = use_signal(|| 0);

    // 1 run of this function = 1 render of this component
    // So, everytime the `count` signal is mutated, the component rerenders.

    rsx!(
        label {
            onclick: move |_| count += 1,
            "Increase {count}"
        }
    )
}
```