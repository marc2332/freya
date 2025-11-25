This is a fork of [Dioxus Router](https://github.com/DioxusLabs/dioxus/tree/main/packages/router)

### Main differences

- No root router: Dioxus Router only works with 1 Router per app, if you try more it will create conflicts and ultimately error out. Freya Router in the other hand allows multiple routers in the same tree by simply providing the router context to the children instead of injecting the router context in the root.
- Built-in `ActivableRoute` (moved from freya-components): A simple helper component to tell the inner children that a certain route is the current one, without actually requiring the inner children to even know what freya-router is.
- Built-in `NativeRouter` (move from freya-components): Freya integration for back and forward navigation with the mouse buttons.
- Always use MemoryHistory: No need to use dioxus renderer-agnostic APIs for this.
- Remove `html` feature: This avoids pulling dioxus-html by default.
- Remove any WASM-splitting features: These are not needed for Freya.