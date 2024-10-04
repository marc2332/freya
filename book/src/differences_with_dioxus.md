# Differences with Dioxus

**Freya** is built on top of the **core** crates from Dioxus, this means that you will effectively be creating Dioxus components, using RSX and hooks. 

**But**, thanks to Dioxus being a renderer-agnostic library you will **NOT** be using JavaScript, HTML, OR CSS, or any other abstraction that ends up using one of those or anything close to web.

 Freya does everything on its own when it comes to:
- Elements
- Styling
- Layout
- Events
- Rendering
- Testing
- Built-in components and hooks,
- Editing
- Animating

And more. Dioxus only is only used to run the app components (hooks, lifecycle, state, rsx), **everything else is managed by Freya**.

**Freya is not mean to be drop-in alternative to Dioxus renderers but as GUI library on its own.**

Here is the list of the main differences between Freya and the official Dioxus renderers for Desktop (WebView and Blitz):

| Category                             | Freya            | Dioxus Renderers                |
|--------------------------------------|------------------|---------------------------------|
| **Elements, attributes and events**  | Custom           | HTML                            |
| **Layout** | Custom ([`Torin`](https://github.com/marc2332/freya/tree/main/crates/torin)) | CSS or [`Taffy`](https://github.com/DioxusLabs/taffy) |
| **Styling**                          | Custom                    | CSS                             |
| **Renderer**                         | Skia                      | WebView or WGPU                 |
| **Components library**               | Custom ([`freya-comonents`](https://github.com/marc2332/freya/tree/main/crates/components))  | None, but can use HTML elements and CSS libraries |
| **Devtools**                         | Custom ([`freya-devtools`](https://github.com/marc2332/freya/tree/main/crates/devtools))   | Provided in Webview             |
| **Headless testing runner**          | Custom ([`freya-testing`](https://github.com/marc2332/freya/tree/main/crates/testing))    | None, but there is Playwright and similar   |
