# Differences with Dioxus

**Freya** uses most of the core packages of Dioxus, but not all them.

Main differences:
- Freya has it's own elements namespace, instead of `HTML`.
- Freya uses it's own UI layour library called  [`torin`](`https://github.com/marc2332/freya/tree/main/torin`), instead of [`taffy`](https://github.com/DioxusLabs/taffy).
- Freya uses Skia, instead of webview or other graphics libraries used by Dioxus.
- Freya comes with a set of ready to use components.
- Freya comes with a set hooks, like for animation.
- Freya comes with a headless testing runner.
- Freya comes with an integrated devtools panel.

## Reminder 👇
The fact that Freya comes with certain features doesn't mean that some Dioxus renderers dont't have them, examples:
- Dioxus web/desktop already has a devtools panel as they run in browser/webview.
- Dioxus web/desktop can take full advantage of CSS libraries and HTML elements that browser/webview already provide.