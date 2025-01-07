# Differences with Dioxus

**Freya** is built on top of the **core** crates from Dioxus. This means that you will effectively be creating Dioxus components using RSX and hooks.

However, thanks to Dioxus being a renderer-agnostic library, you will **NOT** be using JavaScript, HTML, CSS, or any other abstraction that ends up using one of those or other web technologies.

Freya does everything on its own when it comes to:
- Elements
- Styling
- Layout
- Events
- Rendering
- Testing
- Built-in components and hooks
- Editing
- Animating

...and more. Dioxus is only used for managing app components (hooks, lifecycle, state, RSX), while **everything else is managed by Freya**.

**Freya is not meant to be a drop-in alternative to Dioxus renderers but a GUI library on its own.**

Below is a comparison of the main differences between Freya and the official Dioxus renderers for Desktop (WebView and Blitz):

| Category                             | Freya            | Dioxus Renderers                |
|--------------------------------------|------------------|---------------------------------|
| **Elements, attributes, and events** | Custom           | HTML                            |
| **Layout** | Custom ([`Torin`](https://github.com/marc2332/freya/tree/main/crates/torin)) | CSS or [`Taffy`](https://github.com/DioxusLabs/taffy) |
| **Styling**                          | Custom                    | CSS                             |
| **Renderer**                         | Skia                      | WebView or WGPU                 |
| **Components library**               | Custom ([`freya-components`](https://github.com/marc2332/freya/tree/main/crates/components)) | None, but can use HTML elements and CSS libraries |
| **Devtools**                         | Custom ([`freya-devtools`](https://github.com/marc2332/freya/tree/main/crates/devtools))   | Provided in WebView              |
| **Headless testing runner**          | Custom ([`freya-testing`](https://github.com/marc2332/freya/tree/main/crates/testing))       | None, but tools like Playwright and similar are available |
