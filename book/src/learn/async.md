# Async

You may run asynchronous code through the different APIs Dioxus provide. You can use other libraries such as tokio to spawn tasks but it's not always the recommended approach as these will not work with the lifecycling of the components.


### `spawn`

Simple function to spawn an **async task**, this is primarily targeted for custom hooks or when you wanted to run some async code dynamically from an event listener.

Important to know: Tasks spawned with `spawn` will be cancelled when the component their were created is dropped. If you want to have an async tasks not attached to the component you may use `spawn_forever`.

```rs
fn app() -> Element {
    rsx!(
        Button {
            onclick: |_| {
                if 1 == 1 {
                    spawn(async move {
                        println!("Hello, World fom an async task!");
                    });
                }
            }
        }
    )
}
```

### `use_future`

TODO

### `use_resource`

TODO