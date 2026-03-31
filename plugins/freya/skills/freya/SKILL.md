---
name: freya
description: Freya Rust GUI framework best practices, patterns, and conventions. Use when writing Freya components, hooks, elements, or working on a Freya project.
user-invocable: true
---

# Freya Best Practices

Freya is a cross-platform, native, declarative GUI library for Rust.

## Components

### Struct Components (for stateful UI)

```rust
#[derive(PartialEq)]
struct Counter {
    initial: i32,
}

impl Component for Counter {
    fn render(&self) -> impl IntoElement {
        let mut count = use_state(|| self.initial);
        label()
            .on_mouse_up(move |_| *count.write() += 1)
            .text(format!("Count: {}", count.read()))
    }
}
```

- `#[derive(PartialEq)]` is required - Freya uses it to skip re-rendering unchanged subtrees.
- Implement `KeyExt` and `ChildrenExt` when the component can be keyed or accept children.

### Function Components (app root or stateless helpers)

```rust
fn app() -> impl IntoElement {
    rect().child("Hello, World!")
}
```

Pass data from `main` via the `App` trait:

```rust
struct MyApp { number: u8 }

impl App for MyApp {
    fn render(&self) -> impl IntoElement {
        label().text(self.number.to_string())
    }
}
```

### Utility Functions (stateless, no hooks needed)

```rust
fn colored_label(color: Color, text: &str) -> impl IntoElement {
    label().color(color).text(text.to_string())
}
```

Use plain functions when you only need to reuse a chunk of UI with no internal state. Use a `Component` when you need hooks or render optimization.

## Element Builder Pattern

Elements use a fluent builder API. **Never store an element in a variable to modify it later** - chain all methods directly or use `.when` / `.map`.

```rust
// Good
rect()
    .background((255, 0, 0))
    .width(Size::fill())
    .height(Size::px(100.))
    .center()       // centers children both axes
    .expanded()     // fills available space in parent's main axis
    .when(is_active, |r| r.child("Active"))
    .map(|r| r.expanded())

// Bad - storing to modify later
let mut element = rect();
```

Common layout shorthands: `.center()` centers children on both axes; `.expanded()` makes the element fill all remaining space along the parent's main axis (equivalent to `flex: 1` in CSS).

### Conditional and Dynamic Rendering

```rust
rect()
    .when(show_badge, |r| r.child("New"))
    .map(|r| if large { r.height(Size::px(200.)) } else { r })
    .maybe_child(optional_element) // appends only when Some
```

`.maybe_child(Option<impl IntoElement>)` is the idiomatic way to conditionally append a child that may or may not exist.

### Labels from &str and String

`&str` and `String` implement `Into<Label>`, so prefer passing them directly instead of constructing a `label()`:

```rust
rect().child("Hello")               // preferred
rect().child(label().text("Hello")) // unnecessary
```

## Hooks

Hooks are prefixed with `use_` (e.g. `use_state`, `use_animation`). Follow these rules:

1. **Only call hooks at the top level of `render`** - never inside conditionals, loops, or closures.
2. **Hooks must be called in the same order on every render.**
3. **Capture hook values in `move` closures** for event handlers:

```rust
let mut state = use_state(|| false);
let on_click = move |_| state.set(true); // capture, not call inside handler
rect().on_mouse_up(on_click)
```

## State Management

### Local State

```rust
let mut count = use_state(|| 0);
*count.write() += 1;    // write
let n = *count.read();  // read
count.set(5);           // convenience setter
```

`use_state` returns a `Copy` type (`State<T>`). No `.clone()` needed when passing it around.

Pass local state to child components:

```rust
#[derive(PartialEq)]
struct Child(State<i32>);
```

### Global State - Freya Radio

Use Freya Radio for large or deeply nested app state where you need surgical, fine-grained updates - only the components subscribed to a specific channel re-render when that channel changes. This makes it well-suited for complex UIs (e.g. a tab system where each tab has independent state, or a big data model where different parts of the UI subscribe to different slices).

Define your state and a channel enum that maps to the parts of the state that can change independently:

```rust
#[derive(Default, Clone)]
struct AppState {
    count: i32,
    name: String,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
enum AppChannel {
    Count,
    Name,
}

impl RadioChannel<AppState> for AppChannel {}
```

Initialize once in the root component, then subscribe from any descendant:

```rust
// Root
use_init_radio_station::<AppState, AppChannel>(AppState::default);

// Any component - only re-renders when AppChannel::Count changes
let mut radio = use_radio(AppChannel::Count);
radio.read().count;
radio.write().count += 1;
```

For channels where a write to one should also notify subscribers of another, override `derive_channel`:

```rust
impl RadioChannel<AppState> for AppChannel {
    fn derive_channel(self, _state: &AppState) -> Vec<Self> {
        match self {
            // Writing to Count also notifies Name subscribers
            AppChannel::Count => vec![self, AppChannel::Name],
            AppChannel::Name => vec![self],
        }
    }
}
```

For complex state transitions, implement the reducer pattern with `DataReducer`:

```rust
impl DataReducer for AppState {
    type Channel = AppChannel;
    type Action = AppAction;

    fn reduce(&mut self, action: AppAction) -> ChannelSelection<AppChannel> {
        match action {
            AppAction::Increment => { self.count += 1; }
            AppAction::SetName(n) => { self.name = n; }
        }
        ChannelSelection::Current
    }
}

// Then in a component:
radio.apply(AppAction::Increment);
```

### Readable / Writable (type-erased abstractions)

Use `Readable<T>` / `Writable<T>` as component props when the component should accept state from any source:

```rust
#[derive(PartialEq)]
struct NameInput { name: Writable<String> }

// Caller passes either local state or radio slice:
NameInput { name: local_name.into_writable() }
NameInput { name: name_slice.into_writable() }
```

### Context API

Use context to make a value available to any descendant component without threading it through every prop. Prefer this over `static` variables, `thread_local!`, or global singletons - context is scoped to the component tree and plays well with Freya's reactivity.

```rust
// Provider: stores the value and makes it available to all descendants
fn app() -> impl IntoElement {
    use_provide_context(|| AppConfig { theme: Theme::Dark });
    rect().child(DeepChild {})
}

// Consumer: retrieve by type, walks up the tree until found
#[derive(PartialEq)]
struct DeepChild;
impl Component for DeepChild {
    fn render(&self) -> impl IntoElement {
        let config = use_consume::<AppConfig>();
        format!("Theme: {:?}", config.theme)
    }
}
```

Use `use_try_consume::<T>()` when the context may not be present. If context is not found, `use_consume` panics.

Context values are identified by type, so each distinct type gets its own slot. Providing the same type again in a deeper component shadows the ancestor's value for that subtree.

Context is the right tool for dependency injection (e.g. passing a DB client, config, or theme down the tree). For reactive shared state use Freya Radio; for passing state between a parent and immediate children, plain props or `State<T>` are simpler.

### Choosing state type

- `use_state` - component-local state
- Context API - dependency injection and non-reactive shared values across the tree; prefer over statics
- Freya Radio - large/nested state, surgical per-channel updates, multi-window
- `Readable`/`Writable` - reusable components that don't care about backing storage

## Derived State and Side Effects

For simple derived values, compute them directly in `render` - no hook needed:

```rust
let doubled = *count.read() * 2;
```

For expensive computations that should only re-run when their dependencies change, use `use_memo`. It subscribes to any `State` read inside the callback and caches the result:

```rust
let expensive = use_memo(move || {
    let n = *count.read(); // subscribed - reruns when count changes
    compute_something(n)
});
let value = expensive.read();
```

For side effects that should re-run when state changes (e.g. logging, triggering external systems), use `use_side_effect`. Do not use it to sync one state into another - derive values directly or use `use_memo` instead:

```rust
use_side_effect(move || {
    let value = *count.read(); // subscribed
    println!("count changed: {value}");
});
```

## Async

### Spawning tasks

Use Freya's `spawn()` (not `tokio::spawn`) for async work that updates the UI. Tasks spawned with `spawn()` are tied to Freya's reactivity system and can safely write to component state:

```rust
let mut data = use_state(|| None);

use_hook(move || {
    spawn(async move {
        let result = fetch_something().await;
        data.set(Some(result));
    });
});
```

`use_hook` runs once on mount, making it the right place for one-shot side effects. `spawn` returns a `TaskHandle` you can cancel if needed.

### Async functions in components

Components and hooks are synchronous - you cannot `await` inside `render`. Spawn a task and store the result in state:

```rust
impl Component for MyComponent {
    fn render(&self) -> impl IntoElement {
        let mut result = use_state(|| String::new());

        use_hook(move || {
            spawn(async move {
                let s = some_async_fn().await;
                result.set(s);
            });
        });

        result.read().as_str()
    }
}
```

### use_future

`use_future` wraps this pattern: it starts an async task on mount and exposes its state as `FutureState<D>` (`Pending`, `Loading`, `Fulfilled(D)`):

```rust
let task = use_future(|| async {
    fetch_user(42).await
});

match &*task.state() {
    FutureState::Pending | FutureState::Loading => "Loading...",
    FutureState::Fulfilled(user) => user.name.as_str(),
}
```

Call `task.start()` to restart and `task.cancel()` to stop it.

### freya-query (cached async data)

For data that should be cached, deduplicated, and automatically refetched, use `freya-query` (`features = ["query"]`):

```rust
// Define the query
#[derive(Clone, PartialEq, Hash, Eq)]
struct FetchUser;

impl QueryCapability for FetchUser {
    type Ok = String;
    type Err = String;
    type Keys = u32;

    async fn run(&self, user_id: &u32) -> Result<String, String> {
        Ok(format!("User {user_id}"))
    }
}

// Use it in a component
impl Component for UserProfile {
    fn render(&self) -> impl IntoElement {
        let query = use_query(Query::new(self.0, FetchUser));

        match &*query.read().state() {
            QueryStateData::Pending => "Loading...",
            QueryStateData::Settled { res, .. } => res.as_deref().unwrap_or("Error"),
            QueryStateData::Loading { .. } => "Refreshing...",
        }
    }
}
```

Multiple components using the same `(capability, keys)` pair share one cache entry. Invalidate with `query.invalidate()` or `QueriesStorage::<FetchUser>::invalidate_all().await`.

For write operations, use `use_mutation` + `MutationCapability`. The `on_settled` callback is the right place to invalidate related queries after a mutation.

Prefer `freya-query` over manual `use_future` + state when you need caching, background refetch, or deduplication.

### Tokio integration

Freya has its own async runtime. To use Tokio-ecosystem crates (`reqwest`, `sqlx`, etc.), enter a Tokio runtime context in `main` before launching:

```rust
fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    let _guard = rt.enter(); // keep alive for the whole program

    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}
```

Use Freya's `spawn()` for UI updates. `tokio::spawn` runs on the Tokio runtime and **cannot** update component state.

## Keying

Use `.key(id)` on elements in dynamic lists to ensure correct reconciliation on reorders:

```rust
VirtualScrollView::new(|i, _| {
    rect()
        .key(i)
        .child(format!("Item {i}"))
        .into()
})
.length(items.len())
```

Missing `.key()` in dynamic lists causes element misidentification during reorders.

## Internationalization (freya-i18n)

Enable with `features = ["i18n"]`. Uses [Fluent](https://projectfluent.org/) (`.ftl` files) for translations.

**1. Define `.ftl` files:**

```ftl
# en-US.ftl
hello_world = Hello, World!
hello = Hello, { $name }!
```

**2. Initialize once in the root component:**

```rust
use freya::i18n::*;

let mut i18n = use_init_i18n(|| {
    I18nConfig::new(langid!("en-US"))
        .with_locale(Locale::new_static(langid!("en-US"), include_str!("../i18n/en-US.ftl")))
        .with_locale(Locale::new_static(langid!("es-ES"), include_str!("../i18n/es-ES.ftl")))
        .with_fallback(langid!("en-US"))
});
```

**3. Translate in any descendant component:**

```rust
// t! panics if key missing, te! returns Result, tid! falls back to the key string
t!("hello_world")                  // "Hello, World!"
t!("hello", name: {"Alice"})       // "Hello, Alice!"
te!("hello_world")                 // Ok("Hello, World!")
tid!("missing-key")                // "message-id: missing-key should be translated"
```

**4. Switch language at runtime:**

```rust
let mut i18n = I18n::get(); // retrieve from any descendant
i18n.set_language(langid!("es-ES"));
```

For multi-window apps, create with `I18n::create_global` in `main` and share with `use_share_i18n`.

## Animations

Use `use_animation` for manual control and `use_animation_transition` to animate between two values reactively:

```rust
// Manual: call .start() / .reverse() yourself
let mut anim = use_animation(|_| AnimColor::new((240, 240, 240), (200, 80, 80)).time(400));
rect().background(&*anim.read()).on_press(move |_| anim.start())

// Transition: re-runs automatically when the tracked value changes
let color = use_animation_transition(is_active, |from, to| AnimColor::new(from, to).time(300));
rect().background(&*color.read())
```

Animate colors (`AnimColor`), sizes, positions, and other numeric properties. Easing functions and sequencing are supported.

## Routing

Enable with `features = ["router"]`. Define routes with `#[derive(Routable)]`, render them with `router::<Route>()`, place the current page with `outlet::<Route>()`, and navigate with `Link` or `RouterContext::get().replace(...)`:

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Home,
    #[route("/settings")]
    Settings,
}

fn app() -> impl IntoElement {
    router::<Route>(|| RouterConfig::default())
}
```

## Headless Testing

`freya-testing` lets you test components without a window. Use `TestingRunner` to mount a component, simulate interactions, and assert on state:

```rust
use freya_testing::prelude::*;

let (mut runner, state) = TestingRunner::new(app, (300., 300.).into(), |r| {
    r.provide_root_context(|| State::create(0))
}, 1.);

runner.sync_and_update();
runner.click_cursor((15., 15.));
assert_eq!(*state.peek(), 1);
```

Call `runner.render_to_file("out.png")` to snapshot the current UI.

## Icons

Enable with `features = ["icons"]`. Uses Lucide icons rendered as SVGs:

```rust
use freya::icons;

svg(icons::lucide::antenna()).color((120, 50, 255)).expanded()
```

## Rich Text Editing

Use `use_editable` to manage a text editor with cursor, selection, keyboard shortcuts, and virtualization. Wire it to a `paragraph()` element's event handlers and feed `EditableEvent`s from mouse/keyboard events. See `examples/` for full wiring.

## Code Editor

Enable with `features = ["code-editor"]`. `CodeEditorData` holds a `Rope`-backed buffer with tree-sitter syntax highlighting. Pass it to the `CodeEditor` component:

```rust
let editor = use_state(|| {
    let mut e = CodeEditorData::new(Rope::from_str(src), LanguageId::Rust);
    e.parse();
    e.measure(14., "Jetbrains Mono");
    e
});
CodeEditor::new(editor, focus.a11y_id())
```

## Plotting

Enable with `features = ["plot"]`. Use the `plot()` element with a `RenderCallback` and draw into it using the Plotters API via `PlotSkiaBackend`:

```rust
plot(RenderCallback::new(|ctx| {
    let backend = PlotSkiaBackend::new(ctx.canvas, ctx.font_collection, size).into_drawing_area();
    // ... Plotters drawing code
})).expanded()
```

## Material Design

Enable with `features = ["material-design"]`. Adds style modifiers like `.ripple()` to built-in components:

```rust
use freya::material_design::*;
Button::new().ripple().child("Click me")
```

## WebView

Enable with `features = ["webview"]`. Embeds a browser view into your UI:

```rust
use freya::webview::*;
WebView::new("https://example.com").expanded()
```

## Terminal

Enable with `features = ["terminal"]`. Spawns a PTY process and renders it as a terminal:

```rust
use freya::terminal::*;
let mut cmd = CommandBuilder::new("bash");
cmd.env("TERM", "xterm-256color");
let handle = TerminalHandle::new(TerminalId::new(), cmd, None).ok();
// Render with Terminal::new(handle) and forward keyboard events via handle.write_key()
```

## Developer Tools

Enable with `features = ["devtools"]`. Adds a real-time component tree inspector. Run the devtools app alongside your app to examine layout, props, and state.

## Crate Features

Add to your `Cargo.toml` as needed:

```toml
freya = { version = "...", features = ["router", "radio"] }
```

| Feature | What it enables |
|---|---|
| `router` | Page routing (`freya-router`) |
| `i18n` | Internationalization via Fluent (`freya-i18n`) |
| `remote-asset` | Load images/assets from remote URLs |
| `radio` | Global state management (`freya-radio`) |
| `query` | Async data fetching with caching (`freya-query`) |
| `sdk` | Generic utility APIs (`freya-sdk`) |
| `plot` | Chart/plotting via Plotters (`freya-plotters-backend`) |
| `gif` | Animated GIF support in `GifViewer` |
| `calendar` | `Calendar` date-picker component |
| `markdown` | `Markdown` renderer component |
| `icons` | SVG icon library via Lucide (`freya-icons`) |
| `material-design` | Material Design theme (`freya-material-design`) |
| `webview` | Embed a WebView (`freya-webview`) |
| `terminal` | Terminal emulator (`freya-terminal`) |
| `code-editor` | Code editing APIs (`freya-code-editor`) |
| `tray` | System tray support |
| `titlebar` | Custom window titlebar component |
| `devtools` | Developer tools overlay |
| `performance` | Performance monitoring plugin |
| `hotpath` | Hot-path optimization |
| `all` | All of the above (except devtools/performance/hotpath) |

## Further Reference

- `AGENTS.md` (also symlinked as `CLAUDE.md`) in the repo root - authoritative dev workflow and Rust conventions for working on Freya itself.
- `crates/freya/src/_docs/` - in-source documentation for hooks, state management, components, routing, animations, and more.
- `examples/` - 150+ working examples covering every feature.
