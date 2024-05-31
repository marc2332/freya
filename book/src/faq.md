# Frequently Asked Questions

## How is this different from Dioxus?

See the [differenes](./differences_with_dioxus.md).

## Will Freya have Mobile/Web support?
Freya's current focus is on Desktop (Windows, Linux, MacOS), so there are currently no plans to support either Mobile (Android/iOS) or Web platforms. But, this doesn't mean it won't happen in the future, who knows! From a technical point of view, it is possible to run Freya on these platforms with the right adjustments.

## Why choose Skia instead of Webview?
These are the main reasons for this:
- Ability to define the elements, attributes, styling, layout and events to my own criteria
- App UIs look the same across platforms
- Because Freya has control over the entire pipeline, it is easier to implement and use certain features such as headless testing runners
- No reliance on OS for new features or fixes
