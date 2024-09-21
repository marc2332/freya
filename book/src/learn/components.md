# Components

Freya apps will usually be composed of different components.
Components are defined in the form functions that might receive some input as **Props** and return the UI as **Element**.

> You can learn more about how the UI is defined in the [UI](./ui.md) chapter.

This is how a simple root component looks like:

```rs
// Usually, the root component of a Freya app is named `app`, 
// but it is not a requirement
fn app() -> Element {
    rsx!(
        label {
            "Hello, World!"
        }
    )
}
```

This is obviously fine, but the moment our app grows in size and complexity we might want to split
things out in order to maintain a certain level of modularity and reusability. We can do this by spliting the UI in different components

For example, lets create a reusable component:

```rs
fn app() -> Element {
    rsx!(
        // By declaring this element using `TextLabel`
        // we are creating an instance of that component 
        TextLabel {
            "Number 1"
        }
        label {
            "Number 2"
        }
        // Another instance of the same component
        TextLabel {
            "Number 3"
        }
    )
}

// Reusable component that we might call as many times we want
#[component]
fn TextLabel(text: String) -> Element {
    rsx!(
        label {
            "{text}"
        }
    )
}
```

Notice how we anotate our `TextLabel` component with the macro `#[component]`, this will transform every argument of the function (just `text: String` in this case) to a component prop, so we can later use the component prop in a declarative way in the RSX.

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

Components renders are just when a component function runs, which might be because it is subscribed to a signal and that signal got mutated, or because its props changed.

> Even though the naming might give you the impression that it means the app will effectively rerender again, it has nothing to do with it, in fact, a component might render a thousand times but it it doesn't generate a new UI Freya will not rerender it.

Consider this simple component:

```rs
#[component]
fn CoolComp() -> Element {
    let mut count = use_signal(|| 0);

    // 1 run of this function means 1 render of this component
    // So, everytime the `count` signal is mutated, the component rerenders/is recalled.

    rsx!(
        label {
            // Update the signal value
            onclick: move |_| count += 1,

            // By embedding the count in this text the component is subscried to any change in the `count` siganal
            "Increase {count}"
        }
    )
}
```