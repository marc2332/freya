# Architecture
Freya is a UI library build on top of Dioxus core and powered by Skia.

## Overview

### `crates/common`
This crate simply contains types used across the rest of the crates, these are placed here to avoid conflicts.

### `crates/components`

This crate contains the built-in components which will be used by the users to 
build their apps.


### `crates/core`
The core crates manages:

- Accessibility integration with `AccessKit` in a 
platform-agnostic way, meaning that it doesn't make use of accesskit-winit, 
that job is left for the renderer crate.

- Receive events from the platform, then process them and output the 
the events that will be emitted to the VirtualDOM, and therefore, received by
the elements.

- Layout integration with Torin (Layout Library)

- Viewports creation

- Per layer ordering of elements

### `crates/devtools`

Built-in panel with developer tools, which are enabled by the `devtools` feature in the `freya` crate. 

This panel is made entirely with Freya itself, so it runs on Dioxus components, and it uses the same VirtualDOM, RealDOM and Layout as the user app.

### `crates/dom`
Constains an abstraction over the Dioxus RealDOM and provides an adapter for Torin so it can interact with the RealDOM.

### `crates/elements`
Has the definitions of all elements, attributes,  events and events datas.


### `crates/engine`
Proxy to the Skia library to allow mocking all the apis so freya can be built on docs.rs.

### `crates/freya`
Entrypoint to the library, it contains launch functions to run the app into the native renderer.

### `crates/hooks`
Built-in hooks designed to be used in Freya.

### `crates/renderer`
Native render using a Skia canvas in a winit window. It also contains an accessibility integration with accesskit-winit.

### `crates/state`
Definitions of all the internal states that Nodes have, for example, Style state, layout state. It also serves as the place to implement parsing functions for those attributes that have special syntax, like size attributes (`width`, `height`,etc).

### `crates/testing`
Testing framework to run Freya components in a headless environment.

### `crates/torin`
Torin is a completely agnostic library to create UI layouts.

### `book`
The Official Book of Freya, it contains an overview of the library and guides.

### `website`
Freya's official Website, made with Astro.