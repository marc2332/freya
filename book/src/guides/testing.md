# Testing

Freya comes with a special testing renderer that let's you run your component in a headless environment. This will let you write unit tests for your components.

## Getting started

This will launch a state-less component and assert that it renders a label with the text "Hello World!". Just like the normal renderer, you have a launch function where you pass your component. This will return you a set of utilities for you to interact with the component.

```rust
#[tokio::test]
async fn test() {
    fn our_component(cx: Scope) -> Element {
        render!(
            label {
                "Hello World!"
            }
        )
    }

    let mut utils = launch_test(our_component);

    let root = utils.root();
    let label = root.get(0);
    let label_text = label.get(0);

    assert_eq!(label_text.text(), Some("Hello World!"));
}
```

The `root()` function will give you the Root node of your app, then, with the `get` function you can retrieve a Node from it's parent given it's index position.

## Dynamic components

For dynamic components we will need to poll the event loop with the `wait_for_update` function.

```rust
#[tokio::test]
async fn dynamic_test() {
    fn dynamic_component(cx: Scope) -> Element {
        let state = use_state(cx, || false);
        let state_setter = state.setter();

        use_effect(cx, (), move |_| async move {
            state_setter(true);
        });

        render!(
            label {
                "Is enabled? {state}"
            }
        )
    }

    let mut utils = launch_test(dynamic_component);

    let root = utils.root();
    let label = root.get(0);

    assert_eq!(label.get(0).text(), Some("Is enabled? false"));

    utils.wait_for_update().await;

    assert_eq!(label.get(0).text(), Some("Is enabled? true"));
}
```