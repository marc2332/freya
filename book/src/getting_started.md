# Getting Started

I encourage you to first learn about [Dioxus](dioxus.html), when you are done you can continue here. Also make sure you have the followed the [Setup](./setup.html) guide.

Now, let's start by creating a hello world project.

### Creating the project

```sh
mkdir freya-app
cd freya-app
cargo init
```

### Cargo.toml

Let's add Freya and Dioxus, but this last one with only 2 features selected as we don't want to pull unnecessary dependencies.

```toml
[package]
name = "freya-app"
version = "0.1.0"
edition = "2021"

[dependencies]
freya = "0.2"
dioxus = { version = "0.5", features = ["macro", "hooks"], default-features = false }
```

### src/main.rs

You pass your root (usually named `app`) component to the `launch` function, which will open the window and then render your component.

```rust, no_run
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app); // Be aware that this will block the thread
}
```

Let's define our root component:

```rust, no_run
fn app() -> Element {

    // RSX is a special syntax to define the UI of our components
    // Here we simply show a label element with some text
    rsx!(
        label { "Hello, World!" }
    )
}
```

Components in Freya are just functions that return an `Element` and might receive some properties as arguments.

Let's make it stateful by using Signals from Dioxus:

```rust, no_run
fn app() -> Element {
    // `use_signal` takes a callback that initializes the state
    // No matter how many times the component re runs, 
    // the initialization callback will only run once at the first component run
    let mut state = use_signal(|| 0); 

    // Because signals are copy, we can move them into closures
    let onclick = move |_| {
        // Signals provide some mutation shortcuts for certain types
        state += 1;
        // But we could do as well
        *state.write() += 1;
    };

    // You subscribe to a signal by calling it (`signal()`), 
    // calling the `read()` method, or just embedding it into the RSX.
    // Everytime the signal is mutated the component function will rerun
    // because it has been subscribed, this will end up producing a 
    // new `Element` with the updated counter.
    println!("{}", state());
    println!("{}", state.read());

    rsx!(
        label { 
            onclick,
            "State is {state}"
         }
    )
}
```

### Running
Simply run with `cargo`:

```sh
cargo run
```

Nice! You have created your first Freya app. 

You can learn more with the [examples](https://github.com/marc2332/freya/tree/main/examples) in the repository.