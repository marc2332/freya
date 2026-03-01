# Contributing to Freya

Hey! I hope this guide can help you contribute to Freya. If you simply have question or issues you can go to the [Discussions Tab](https://github.com/marc2332/freya/discussions) or the [Issues Tracker](https://github.com/marc2332/freya/issues).

## Commands

Running an example from the `/examples` folder:
```sh
cargo run --example counter
```

For this you will need [just](https://github.com/casey/just).

Running the linter:
```sh
just c
```

Running rustfmt in all the workspace:
```sh
just f
```

Running tests:
```sh
just t
```

Check the `justfile` for other commands.

## Architecture Overview

Freya is split in various crates, each with it's own meaning and purpose, here is the list sorted by their importance:

- `freya`: Entrypoint to the library used by end users, mainly reexports the other crates.
- `freya-winit`: Window renderer for Freya.
- `freya-testing`: Headless renderer for Freya, used for testing.
- `freya-core`: Reactivity system, elements tree, hooks, etc.
- `torin`: UI layout library specifically made for Freya, although it's agnostic.
- `ragnarok`: UI events measurer (e.g what event to figure when the user clicks in a coordinate).
- `pathgraph`: Map to store nested data structures based on the assumption that you always know its location.
- `freya-components`: Collection of components ready to be used out of the box with in Freya apps (Button, Switch, Slider, Table, ScrollView, etc)
- `freya-engine`: Simple re-export and mock of all Skia APIs used in freya so Freya docs can be built on docs.rs.
- `freya-devtools`: Devtools server and plugin for Freya.
- `freya-devtools-app`: Standalone Freya app to debug or inspect your Freya apps, its powered by `freya-devtools`.
- `freya-radio`: Global reactive state management based on Topics.
- `freya-i18n`: Language translation library for Freya powered by the Fluent Project.
- `freya-edit`: Text Editing capabilities to create from simple to complex text editors.
- `freya-animation`: Animate numeric or color values for your components.
- `freya-performance-plugin`: Renders a small overlay in the top left corner of your app showing different stats, like FPS, frame time, layout time, tree time, etc.
- `freya-clipboard`: Provides a os-backed clipboard for you to read from and write to.
- `freya-router`: Fully typed router API to manage multiple pages in your app.
- `freya-router-macro`: Macros for `freya-router`.
- `freya-icons`: Provides lots of SVG icons as Freya components.
- `freya-sdk`: Contains generic utility APIs for Freya, like integrations with Tokio.
- `freya-query`: Fully-typed, async, reusable cached data management for Freya apps.
- `freya-webview`: WebView support for Freya using WRY.
- `freya-terminal`: Terminal emulator integration for embedding interactive terminals in Freya apps.
- `freya-material-design`: Material Design Components for Freya apps.
- `freya-plotters-backend`: Freya's skia-safe backend for plotters.
- `freya-code-editor`: Set of APIs to create text Code Editors in Freya.

## Examples
All important examples are located in the  `./examples` folder although you might also find some in the form of docs comments in the code itself.

## Website
The https://freyaui.dev source code is located inside the `./website` folder and is made with [Astro](https://astro.build/).

## Donations
You might also want to sponsor the development of this project through my [Github Sponsor](https://github.com/sponsors/marc2332).
