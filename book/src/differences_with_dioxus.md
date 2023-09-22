# Differences with Dioxus

**Freya** uses most of the core packages of Dioxus, but not all them.

These are the main differences between Freya and the different Dioxus renderers for Desktop (webview and Blitz):

| Category                             | Freya            | Dioxus                          |
|--------------------------------------|------------------|---------------------------------|
| **Elements, attributes and events**  | Custom           | HTML                            |
| **Layout**                           | Torin ([repository](https://github.com/marc2332/freya/tree/main/crates/torin) - [crates.io](https://crates.io/crates/torin))   | WebView and [`taffy`](https://github.com/DioxusLabs/taffy)               |
| **Renderer**                         | Skia             | WebView or WGPU                 |
| **Components library**               | Custom           | None, but can use CSS libraries |
| **Devtools**                         | Custom           | Provided in Webview             |
| **Headless testing runner**          | Custom           | None                            |
