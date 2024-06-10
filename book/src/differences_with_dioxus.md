# Differences with Dioxus

**Freya** is built on top of the core crates from Dioxus, this means that you will effectively be creating Dioxus components, using RSX and hooks. But, you will **not** be using HTML, CSS, JS or any Web tech at all.

Here you can find a list of the main differences between Freya and the official Dioxus renderers for Desktop (WebView and Blitz):

| Category                             | Freya            | Dioxus Renderers                |
|--------------------------------------|------------------|---------------------------------|
| **Elements, attributes and events**  | Custom           | HTML                            |
| **Layout** | [`Torin`](https://github.com/marc2332/freya/tree/main/crates/torin) | CSS or [`Taffy`](https://github.com/DioxusLabs/taffy) |
| **Styling**                          | Custom           | CSS                             |
| **Renderer**                         | Skia             | WebView or WGPU                 |
| **Components library**               | Custom           | None, but can use HTML elements and CSS libraries |
| **Devtools**                         | Custom           | Provided in Webview             |
| **Headless testing runner**          | Custom           | None, but there is Playwright and similar   |
