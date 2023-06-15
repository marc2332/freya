# Differences with Dioxus

**Freya** uses most of the core packages of Dioxus, but not all them.

Main differences:
- Freya has it's own elements, attributes and events, instead of `HTML`.
- Freya uses it's own UI layout library called  [`torin`](`https://github.com/marc2332/freya/tree/main/torin`), instead of [`taffy`](https://github.com/DioxusLabs/taffy).
- Freya uses Skia, instead of webview or other graphics libraries used by Dioxus.
- Freya comes with it's own set of components and hooks, in the other hand Dioxus web/desktop can take advantage of existing CSS libraries and HTML elements.
- Freya has an integrated devtools panel, dioxus web/desktop already have the browser/webview devtools.
- Freya comes with a headless testing runner.
