# What is Freya?

**Freya** is native **GUI** library for Rust🦀, built on top of 🧬 [Dioxus](https://dioxuslabs.com)'s core and powered by 🎨 [Skia](https://skia.org/) as a graphics library.

### Features
- ⛏️ Built-in **components** (button, scroll views, switch and more) 
- 🚇 Built-in **hooks** library (animations, text editing and more)
- 🔍 Built-in **devtools** panel (experimental ⚠️)
- 🧰 Built-in **headless testing** runner for components
- 🎨 **Theming** support (not extensible yet ⚠️)
- 🛩️ Cross-platform (Windows, Linux, MacOS)
- 🖼️ SKSL **Shaders** support
- 🔄️ Dioxus **Hot-reload** support
- 📒 Multi-line **text editing** (experimental ⚠️)
- 🦾 Basic **Accessibility** Support (experimental ⚠️)
- 🧩Compatible with dioxus-std and other Dioxus renderer-agnostic libraries

### Why 🧬 Dioxus?

[Dioxus](https://dioxuslabs.com) is a React-like library for Rust. Its component and hooks model make it simple to use and scale to complex apps. 
Freya uses some of Dioxus core crates to build it's own renderer, this is because Dioxus is to it's core a renderer-agnostic UI library.
See other [differences](./differences_with_dioxus.html) with Freya.

### Why 🎨 Skia?

[Skia](https://skia.org/) is a battle-tested and well-maintained graphics library, and there are even some rusty [bindings](https://github.com/rust-skia/rust-skia). 
