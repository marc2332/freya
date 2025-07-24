# Contributing to Freya

Hey! I hope this guide can help you contribute to Freya. If you simply have question or issues you can go to the [Discussions Tab](https://github.com/marc2332/freya/discussions) or the [Issues Tracker](https://github.com/marc2332/freya/issues).

## Basic commands

Running an example from the `/examples` folder:
```sh
cargo run --example counter
```

Running clippy in all the workspace:
```sh
cargo clippy --workspace
```

Running rustfmt in all the workspace:
```sh
cargo +nightly fmt --all -- --error-on-unformatted --unstable-features
```

### Nix

Start a devshell with rust and packages needed to compile:
```sh
nix develop
```

Start a devshell with nightly rust and packages needed to compile:
```sh
nix develop .#unstable
```

## Architecture Overview

Freya is split in various crates, each with it's own meaning and purpose, here is the list sorted by their importance:

- `freya`: Entrypoint to the library used by end users, mainly reexports the other crates and contains the launch methods.
- `winit`: Winit eventloop-based integration with Freya.
- `core`: DOM Tree, Nodes states, accessibility integration, elements rendering, text layout measurement, event loop messages and more  is located here.
- `native-core`: DOM data structure to hold all the nodes with their attribute values and registered event handlers.
- `torin`: UI layout library specifically made for Freya, although it's agnostic.
- `hooks`: Various Dioxus hooks to be used in Freya apps (text editing, animation, theming, etc)
- `components`: Collection of built-in Dioxus components to be used out of the box with in Freya apps (Button, Switch, Slider, Table, ScrollView, etc)
- `elements`: The definitions of all the built-in elements and events.
- `testing`: Headless (non-graphic) renderer to easily test components and hooks.
- `engine`: Simple re-export and mock of all Skia APIs used in freya so Freya docs can be built on docs.rs.
- `devtools`: The Devtools panel used to inspect the DOM and all the Nodes in real time.
- `native-core-macro`: Just some internal macros to be used in `states` so it can be integrated with `native-core`.

![Overview](./.github/overview.png)

## Examples
All important examples are located in the  `./examples` folder although you might also find some in the form of docs comments in the code itself.

## Website
The https://freyaui.dev  source code is located inside the `./website` folder and is made with [Astro](https://astro.build/).

## Donations
You might also want to sponsor the development of this project through my [Github Sponsor](https://github.com/sponsors/marc2332).
