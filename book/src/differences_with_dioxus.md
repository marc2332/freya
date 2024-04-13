# Differences with Dioxus

**Freya** uses some of the core packages of Dioxus, but not all of them.

These are the main differences between Freya and the official Dioxus renderers for Desktop (WebView and Blitz):

| Category                             | Freya            | Dioxus Renderers                |
|--------------------------------------|------------------|---------------------------------|
| **Elements, attributes and events**  | Custom           | HTML                            |
| **Layout** | [`Torin`](https://github.com/marc2332/freya/tree/main/crates/torin) | CSS or [`Taffy`](https://github.com/DioxusLabs/taffy) |
| **Styling**                          | Custom           | CSS                             |
| **Renderer**                         | Skia             | WebView or WGPU                 |
| **Components library**               | Custom           | None, but can use HTML elements and CSS libraries |
| **Devtools**                         | Custom           | Provided in Webview             |
| **Headless testing runner**          | Custom           | None, but there is Playwright and similar   |
