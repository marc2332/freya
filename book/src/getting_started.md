# Getting Started

I encourage you to learn how [Dioxus works](https://dioxuslabs.com/learn/0.5/guide/your_first_component), when you are done you can continue here. Also make sure you have the followed the [environment setup](../setup.html) guide.

Now, let's start by creating a hello world project.

### Creating the project

```sh
mkdir freya-app
cd freya-app
cargo init
```

### Cargo.toml

Make sure to add Freya and Dioxus as dependencies:

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

In Freya, your run your app by calling a `launch` method and passing your root Component:

```rust, no_run
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app); // This will block the main thread
}
```

Let's define our root component, called `app`:

```rust, no_run
fn app() -> Element {
    render!(
        label { "Hello, World!" }
    )
}
```

Components in Freya/Dioxus are simply functions that return an `Element`.

Let's make it stateful by using Dioxus Signals:

```rust, no_run
fn app() -> Element {
    // `use_signal` takes a callback that initializes the state
    let mut state = use_signal(|| 0); 

    // Because signals are copy, we can move them into closures
    let onclick = move |_| {
        // Signals provide some shortcuts for certain types
        state += 1;
        // But we could do as well
        *state.write() += 1;
    };

    // You can subscribe to a signal, by calling it (`signal()`), 
    // calling the `signal.read()` method, or just embedding it into the RSX.
    // and thus producing a new Element with the updated counter.
    // So, everytime the signal is mutated the component function will rerun
    // because it has been subscribed
    println!("{}", state());

    render!(
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