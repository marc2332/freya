# Components

Freya apps will usually be composed of different components.
Components are defined in teh form functions that might receive some input as **Props** and return the UI as **Element**.

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
things out in order to maintain a certain level of modularity and even reusability. We can do this with components.

For example:

This is how a simple root component looks like:

```rs
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

For more complex components you might want to leave the props to an external props intead of using the `#[components]` macro:

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
