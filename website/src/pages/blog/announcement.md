---
title: 'Freya Announcement'
date: 2023-09-02
description: 'Initial release of Freya. A native GUI library for Rust.'
author: 'marc2332'
layout: ../../layouts/BlogPostLayout.astro
---
## hey 👋

I'm [Marc](https://github.com/marc2332/) and I am happy to announce the first alpha (**v0.1**) of [**Freya**](https://github.com/marc2332/freya), an experimental cross-platform native GUI library for 🦀 [Rust](https://www.rust-lang.org/), built on top of **🧬** [**Dioxus**](https://dioxuslabs.com/) and powered by the 🖼️ [Skia](https://skia.org/) library.

* Website: [freyaui.dev/](https://freyaui.dev/)
    
* Source Code: [github.com/marc2332/freya](http://github.com/marc2332/freya)
    
* Book: [book.freyaui.dev/](https://book.freyaui.dev/)
    
* Discord: [https://discord.gg/sYejxCdewG](https://discord.gg/sYejxCdewG)
    
* Stable API docs: [docs.rs/freya/latest/freya/](https://docs.rs/freya/latest/freya/)
    
* Nightly API docs: [docs.freyaui.dev/freya](https://docs.freyaui.dev/freya)
    

### What is Freya 🦀 ?

**Freya** is a GUI library that extends **Dioxus** by adding its own **renderer** based on **Skia** with the help of a custom layout library, its own **elements** **namespace**, **attributes** and **events**, plus a set of **components** and **hooks**, and also some developer tools like a **Devtools** panel or a **headless components testing runner**.

#### **Do you want to try it?**

Read [this](https://github.com/marc2332/freya#want-to-try-it-). Just be aware that it is still in the ⚠️ experimentation phase, I would like this first release to be an opportunity to gather feedback, suggestions, new contributions and ideas!

### What is Dioxus 🧬 ?

**Dioxus** is a renderer-agnostic UI library for Rust, like React. It uses [**components**](https://dioxuslabs.com/learn/0.4/reference/components) as functions and [**hooks**](https://dioxuslabs.com/learn/0.4/reference/hooks). It supports many renderers: web, backend with SSR, liveview and fullstack, desktop and mobile with webview, desktop with WGPU, or even TUI.

[Learn more about Dioxus.](https://dioxuslabs.com/learn/0.4/guide/your_first_component/) And see the [differences](https://book.freyaui.dev/differences_with_dioxus.html) with Freya.

### Reasons to use Freya ✅

* **Dioxus**: very easy to use, it's blazingly fast and has a bright future.
    
* **Native renderer**: Looks the same on all platforms and avoids compatibility issues
    
* **Built-in Components and hooks:** From a simple Button to a VirtualScrollView or animation utilities.
    
* **Headless testing runner**: Make sure your components work before making a release.
    
* **Languages**: Just Rust!
    

### Reasons to not use Freya ❌

* **It's still experimental**: it might contain bugs, have performance issues or some APIs not being fully usable yet.
    
* **Lack of ecosystem**: You can use renderer-agnostic libraries of Dioxus but it is still a small ecosystem, and non-existent for Freya.
    
* **Lack of docs**: Some things are not fully documented and others could be improved.
    

## Example

Here there is a simple counter app with Freya (source code [here](https://github.com/marc2332/freya/blob/main/examples/counter.rs)):

```rust
fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render!(
        rect {
            height: "20%",
            width: "100%",
            background: "rgb(233, 196, 106)",
            padding: "12",
            color: "rgb(20, 33, 61)",
            label { 
                font_size: "20", 
                "Number is: {count}"
            }
        }
        rect {
            height: "80%",
            width: "100%",
            background: "rgb(168, 218, 220)",
            color: "black",
            padding: "12",
            onclick: move |_| count += 1,
            label { "Click to increase!" }
        }
    )
}
```

![Image description](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/xul317o98e0kjhs7fqek.png align="center")

## ✨ Supported features

* ⛏️ Built-in components (button, scroll views, switch and more)
    
* 🚇 Built-in hooks library (animations, text editing and more)
    
* 🔍 Built-in devtools panel (experimental ⚠️)
    
* 🧰 Built-in headless testing runner for components
    
* 🎨 Theming support (not extensible yet ⚠️)
    
* 🛩️ Cross-platform (Windows, Linux, MacOS)
    
* 🖼️ SKSL Shaders support
    
* 🔄️ Dioxus Hot-reload integration
    
* 📒 Multi-line text editing (experimental ⚠️)
    
* 🦾 Basic Accessibility Support (experimental ⚠️)
    
* 🧩Compatible with dioxus-std and other Dioxus renderer-agnostic libraries
    

## 💻 Supported platforms

All major desktop OS:

* Windows
    
* Linux
    
* macOS
    

It could technically run on more platforms, like Mobile or Web via Wasm, feel free to contribute 😁

## 🔄 Hot reload

Freya supports **Dioxus's hot reload**, which means that you can write and update the **layout**, **styling** and other static **attributes** of your components without having to recompile any rust code, it updates on the fly.

%[https://twitter.com/mkenzo_8/status/1631956848176668672?s=20] 

## 🧰 Testing

Freya supports **headless testing** of components, you can simulate from the window size to events, like mouse or keyboard, and also assert the layout or text values of your components.

Simple example:

```rust
#[tokio::test]
async fn no_state() {
    fn no_state_app(cx: Scope) -> Element {
        render!(
            label {
                "Hello"
            }
        )
    }

    let mut utils = launch_test(no_state_app);

    assert_eq!(utils.root().get(0).get(0).text(), Some("Hello"));
}
```

A more complex example that even simulates click events:

```rust
#[tokio::test]
async fn simulate_events() {
    fn stateful_app(cx: Scope) -> Element {
        let enabled = use_state(cx, || false);
        render!(
            rect {
                width: "100%",
                height: "100%",
                background: "red",
                direction: "both",
                onclick: |_| {
                    enabled.set(true);
                },
                label {
                    "Is enabled? {enabled}"
                }
            }
        )
    }

    let mut utils = launch_test(stateful_app);

    let rect = utils.root().get(0);
    let label = rect.get(0);

    // Inital render
    utils.wait_for_update().await;

    let text = label.get(0);

    assert_eq!(text.text(), Some("Is enabled? false"));

    utils.push_event(FreyaEvent::Mouse {
        name: "click".to_string(),
        cursor: (5.0, 5.0).into(),
        button: Some(MouseButton::Left),
    });

    // New render after clicking
    utils.wait_for_update().await;

    let text = label.get(0);

    assert_eq!(text.text(), Some("Is enabled? true"));
}
```

## 🔍 DevTools

Freya integrates an experimental **DevTools panel** to help you navigate and inspect the elements of your app while you are developing. It is not included in release builds.

You can do:

* Inspect the DOM
    
* Inspect element's styles
    
* Inspect the element's computed layout
    

![](https://user-images.githubusercontent.com/38158676/257818618-fefcb5be-cdc9-4b9a-aa13-0abe67527736.png align="center")

## 🤓 Complex examples

For simpler examples see the Freya [repository](https://github.com/marc2332/freya/tree/main/examples).

[**Freya-editor**](https://github.com/marc2332/freya-editor): An experimental code editor.

![Demo](https://github.com/marc2332/freya-editor/raw/main/demo.png align="left")

[**Canvas**](https://github.com/marc2332/freya/blob/main/examples/canvas.rs): A canvas for floating editors that you can drag around.

![Image description](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/ys0dpqzunwzqmaof73pi.png align="left")

## 📆 Roadmap

* More **elements**, **components**, **hooks** and **events**.
    
* Better performance, both for **rendering** and **layout**.
    
* Better developer tools (**Devtools panel**, **test runner**, etc)
    
* More and better **documentation**
    

I am aware Freya is not perfect, and it will take some time to be production-ready, but I am sure it will keep getting better.

## 👀 See you soon!

I hope you liked this post and want to try Freya at some point, or even contribute!

Make sure to give [Dioxus](https://dioxuslabs.com/) some love, [Jonathan](https://github.com/jkelleyrtp), [Evan](https://github.com/ealmloff) and the other contributors have made an amazing work with Dioxus, they have been a fundamental piece for Freya to work 🫂.

Also thanks to [Armin](https://github.com/pragmatrix) for his amazing work in [rust-skia](https://github.com/rust-skia/rust-skia) and help with issues and doubts 💪, and [Tropix126](https://github.com/Tropix126) for his work in new styling features like more font customization, better rounded corners, better shadows and a few more things 💯 !

You can leave a star ⭐ in the [repository](https://github.com/marc2332/freya) or sponsor me on [GitHub Sponsors](https://github.com/sponsors/marc2332) if you want 💖. You can join the [**Discord**](https://discord.gg/sYejxCdewG) server or follow me on 🐦 [Twitter](https://twitter.com/mkenzo_8), I usually share the progress I make in Freya.

Thanks!