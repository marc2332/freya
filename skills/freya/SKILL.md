---
name: freya
description: Apply Freya best practices when writing or reviewing Rust GUI components, hooks, events, layout, or themes
---

# Freya best practices

## Index

* **General:** [Choose the cleanup scope](#choose-the-cleanup-scope), [Reuse existing Freya APIs](#reuse-existing-freya-apis)
* **Components:** [Separate UI from logic](#separate-ui-from-logic), [Use structs for reusable components](#use-structs-for-reusable-components), [Require values in component constructors](#require-values-in-component-constructors), [Keep builder chains declarative](#keep-builder-chains-declarative), [Pass plain text directly](#pass-plain-text-directly), [Let layout adapt](#let-layout-adapt)
* **Events:** [Bind event handlers before the UI](#bind-event-handlers-before-the-ui), [Use pointer events for gestures](#use-pointer-events-for-gestures), [Use press for activation](#use-press-for-activation), [Make custom controls accessible](#make-custom-controls-accessible), [Cancel the right event](#cancel-the-right-event)
* **State:** [Call hooks in a stable order](#call-hooks-in-a-stable-order), [Derive values instead of syncing state](#derive-values-instead-of-syncing-state), [Derive values without PartialEq](#derive-values-without-partialeq), [Keep side effects out of render](#keep-side-effects-out-of-render), [Give children only the state access they need](#give-children-only-the-state-access-they-need), [Share dependencies through context](#share-dependencies-through-context), [Scope shared state with Freya Radio](#scope-shared-state-with-freya-radio), [Bind inputs directly](#bind-inputs-directly)
* **Styling:** [Use existing theme colors](#use-existing-theme-colors), [Create a custom app theme](#create-a-custom-app-theme), [Override individual theme fields](#override-individual-theme-fields), [Animate changes in state](#animate-changes-in-state)
* **Performance:** [Key lists by item ID](#key-lists-by-item-id), [Virtualize large lists](#virtualize-large-lists), [Downsample images for display](#downsample-images-for-display), [Choose an image sampling filter](#choose-an-image-sampling-filter), [Tune asset cache retention](#tune-asset-cache-retention), [Rasterize complex SVGs in the background](#rasterize-complex-svgs-in-the-background), [Cache expensive calculations with use_memo](#cache-expensive-calculations-with-use_memo), [Subscribe where values are displayed](#subscribe-where-values-are-displayed), [Skip unchanged state writes](#skip-unchanged-state-writes), [Share remote data with Freya Query](#share-remote-data-with-freya-query), [Keep cached data visible during refresh](#keep-cached-data-visible-during-refresh)
* **App:** [Load data outside render](#load-data-outside-render), [Show honest loading progress](#show-honest-loading-progress), [Spawn tasks from event handlers](#spawn-tasks-from-event-handlers), [Release state guards before awaiting](#release-state-guards-before-awaiting), [Build the Tokio runtime manually](#build-the-tokio-runtime-manually), [Use the router for page navigation](#use-the-router-for-page-navigation), [Open external URLs through Freya](#open-external-urls-through-freya), [Translate complete messages](#translate-complete-messages), [Respect Android system bars and cutouts](#respect-android-system-bars-and-cutouts), [Test through the UI](#test-through-the-ui)

## Choose the cleanup scope

When asked to clean up existing code without a clear scope, ask:

> How thorough should the cleanup be: Slightly, Medium, or Completely?

* **Slightly:** Make the smallest useful changes in the requested code and avoid structural refactors
* **Medium:** Focus on the fixes that matter most and skip cosmetic changes with little benefit
* **Completely:** Review all Rust code in the app and go through this guide one practice at a time without skipping any applicable practice

If the user already specified the scope or asked for a specific fix or feature, proceed without asking

## Reuse existing Freya APIs

Check Freya's components, hooks, and builder methods before writing your own

Use the existing API if it meets the requirement

Write custom code only for behavior the existing API cannot provide

**CORRECT:** Use `.center()` to center children on both axes

```rust
rect().center().child("Ready")
```

**WRONG:** Set both axes to center separately when `.center()` does the same thing

```rust
rect()
    .main_align(Alignment::Center)
    .cross_align(Alignment::Center)
    .child("Ready")
```

Use `.main_align()` and `.cross_align()` separately when the axes need different alignment

## Separate UI from logic

Use this order in every `render` with exactly one blank line between nonempty groups:

1. Hooks in a stable order
2. Derived values needed by handlers
3. Named closures for event handlers
4. Derived values needed only by the view
5. UI builder chain as the final expression

Keep business rules and substantial state transitions in model methods or reducers

**CORRECT:** Separate groups with one blank line

```rust
fn render(&self) -> impl IntoElement {
    let mut count = use_state(|| self.initial);

    let reset_value = self.initial;

    let on_reset = move |_: Event<PressEventData>| count.set(reset_value);

    let caption = format!("Reset count ({})", count.read());

    Button::new().on_press(on_reset).child(caption)
}
```

**WRONG:** Skip the blank lines and inline the handler and caption

```rust
fn render(&self) -> impl IntoElement {
    let mut count = use_state(|| self.initial);
    let reset_value = self.initial;
    Button::new().on_press(move |_: Event<PressEventData>| count.set(reset_value)).child(format!("Reset count ({})", count.read()))
}
```

## Use structs for reusable components

Use plain functions for the app root and stateless helpers without hooks

**CORRECT:** Put props and identity on a component and forward its key with `render_key`

```rust
#[derive(PartialEq)]
struct StatusBadge {
    text: String,
    key: DiffKey,
}

impl KeyExt for StatusBadge {
    fn key_mut(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for StatusBadge {
    fn render(&self) -> impl IntoElement {
        label().theme_color().caption().text(self.text.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
```

**WRONG:** Use a plain helper as a reusable component with its own hooks

```rust
fn status_badge(text: String) -> impl IntoElement {
    let mut visible = use_state(|| true);

    let on_hide = move |_: Event<PressEventData>| visible.set(false);

    rect().maybe(*visible.read(), |el| {
        el.child(Button::new().on_press(on_hide).child(text))
    })
}
```

## Require values in component constructors

Take all required values in `new(...)` or a named constructor like `new_with_data(...)`

Give configurable values sensible defaults and expose them through chainable builder methods

**CORRECT:** Require the title and make the summary optional

```rust
#[derive(PartialEq)]
struct StatusCard {
    title: String,
    summary: Option<String>,
    key: DiffKey,
}

impl StatusCard {
    fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            summary: None,
            key: DiffKey::None,
        }
    }

    fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }
}

impl KeyExt for StatusCard {
    fn key_mut(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for StatusCard {
    fn render(&self) -> impl IntoElement {
        rect()
            .child(self.title.clone())
            .maybe_child(self.summary.clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

fn app() -> impl IntoElement {
    StatusCard::new("Sync status").summary("Up to date")
}
```

**WRONG:** Use an empty title as a default and rely on callers to remember to set it

```rust
impl StatusCard {
    fn new() -> Self {
        Self {
            title: String::new(),
            summary: None,
            key: DiffKey::None,
        }
    }

    fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
}

fn app() -> impl IntoElement {
    StatusCard::new().summary("Up to date")
}
```

## Bind event handlers before the UI

Never inline an event handler, even for one line

Use `EventHandler<T>` for callback props on custom components

**CORRECT:** Pass the named closure

```rust
let on_save = move |_: Event<PressEventData>| save_draft();

Button::new().on_press(on_save).child("Save")
```

**WRONG:** Put the closure inside the builder call

```rust
Button::new().on_press(move |_| save_draft()).child("Save")
```

### Scope shared captures and name handlers by action

When several handlers need owned copies of the same data, clone it inside each handler's definition block and keep the original name

Capture `Copy` handles like `State` directly and name handlers by their action or control instead of numbering them

**CORRECT:** Scope each clone to its handler and name the two press handlers by action

```rust
let on_save = {
    let draft = draft.clone();
    move |_: Event<PressEventData>| save_draft(&draft)
};
let on_preview = {
    let draft = draft.clone();
    move |_: Event<PressEventData>| preview_draft(&draft)
};

rect()
    .child(Button::new().on_press(on_save).child("Save"))
    .child(Button::new().on_press(on_preview).child("Preview"))
```

**WRONG:** Scatter capture variables around the handlers and number the handlers

```rust
let draft_for_save = draft.clone();
let draft_for_preview = draft.clone();
let on_press = move |_: Event<PressEventData>| save_draft(&draft_for_save);
let on_press_2 = move |_: Event<PressEventData>| preview_draft(&draft_for_preview);

rect()
    .child(Button::new().on_press(on_press).child("Save"))
    .child(Button::new().on_press(on_press_2).child("Preview"))
```

## Use pointer events for gestures

**CORRECT:** Share one handler across mouse and touch

```rust
let on_pointer_down = move |_: Event<PointerEventData>| start_drag();

rect().on_pointer_down(on_pointer_down)
```

**WRONG:** Use a mouse event for a gesture that should also work on touch

```rust
let on_mouse_down = move |_: Event<MouseEventData>| start_drag();

rect().on_mouse_down(on_mouse_down)
```

## Use press for activation

**CORRECT:** Use `Button` with `on_press` for click, tap, and keyboard activation

```rust
let on_save = move |_: Event<PressEventData>| save_draft();

Button::new().on_press(on_save).child("Save")
```

**WRONG:** Handle pointer input but not keyboard activation

```rust
let on_save = move |_: Event<PointerEventData>| save_draft();

rect().on_pointer_press(on_save).child("Save")
```

## Make custom controls accessible

Prefer built in controls like `Button`

Expose meaningful content and controls without making every element a tab stop

Skip labels and tab stops for purely decorative icons, spacers, and layout wrappers

Create one stable ID per custom focusable node with `use_a11y()`, attach it with `.a11y_id(...)`, and track focus with `use_focus(id)`

**CORRECT:** Name the icon button and show visible keyboard focus

```rust
let control_id = use_a11y();
let focus = use_focus(control_id);
let theme = use_theme();

let on_save = move |_: Event<PressEventData>| save_draft();

let focus_color = theme.read().color(ColorRole::BorderFocus);

rect()
    .a11y_id(control_id)
    .a11y_role(AccessibilityRole::Button)
    .a11y_alt("Save draft")
    .a11y_focusable(true)
    .on_press(on_save)
    .maybe(focus() == Focus::Keyboard, |el| {
        el.border(Border::new().fill(focus_color).width(2.))
    })
    .child(save_icon())
```

**WRONG:** Make the icon respond only to pointer input

```rust
let on_save = move |_: Event<PointerEventData>| save_draft();

rect().on_pointer_press(on_save).child(save_icon())
```

* `.a11y_enabled(...)` reports enabled state but does not disable handlers
* `.a11y_auto_focus(true)` requests focus on mount
* `.a11y_member_of(group_id)` associates the node with a group
* `.a11y_builder(...)` sets properties not covered by the other methods

## Cancel the right event

**CORRECT:** Stop a child press from reaching the parent with `stop_propagation()`

```rust
let on_child_press = move |event: Event<PressEventData>| {
    event.stop_propagation();
    open_details();
};

rect().on_press(on_parent_press).child(
    rect().on_press(on_child_press).child("Details"),
)
```

**WRONG:** Use `prevent_default()` to stop the same event from bubbling

```rust
let on_child_press = move |event: Event<PressEventData>| {
    event.prevent_default();
    open_details();
};

rect().on_press(on_parent_press).child(
    rect().on_press(on_child_press).child("Details"),
)
```

Use `prevent_default()` to cancel follow up events that the current event would otherwise dispatch

## Use existing theme colors

Start with themed components and their variants

Use theme extensions such as `.theme_background()` on rectangles, `.theme_color()` on text, and `.theme_stroke()` on outlined SVGs

When no extension fits, read a palette role with `theme.read().color(ColorRole::TextSecondary)` instead of using `get_var` with a color key

Initialize the app theme with `use_init_theme(light_theme)` or `use_init_theme(dark_theme)`

**CORRECT:** Use theme extensions and read secondary text from the active palette

```rust
let theme = use_theme();

let secondary_text = theme.read().color(ColorRole::TextSecondary);

rect()
    .theme_background()
    .child(label().theme_color().title().text("Settings"))
    .child(label().color(secondary_text).body().text("Manage your account"))
```

**WRONG:** Hardcode a light theme in the view, even through constants

```rust
const BACKGROUND: Color = Color::WHITE;
const TEXT: Color = Color::BLACK;
const SECONDARY_TEXT: Color = Color::from_rgb(100, 100, 100);

rect()
    .background(BACKGROUND)
    .child(label().color(TEXT).font_size(24.).text("Settings"))
    .child(label().color(SECONDARY_TEXT).font_size(14.).text("Manage your account"))
```

## Create a custom app theme

Start with `light_theme()` or `dark_theme()` and override existing roles with `Theme::set_var` before adding app specific keys

**CORRECT:** Set the brand color once and initialize the custom theme at the app root

```rust
fn brand_theme() -> Theme {
    let mut theme = dark_theme();
    theme.set_var(ColorRole::Primary.key(), Color::from_rgb(37, 52, 63));
    theme
}

fn app() -> impl IntoElement {
    use_init_theme(brand_theme);

    Button::new().filled().child("Save")
}
```

**WRONG:** Repeat the brand color on each filled button

```rust
Button::new()
    .filled()
    .background(Color::from_rgb(37, 52, 63))
    .child("Save")
```

For custom component defaults, declare fields with `define_theme!`, register preferences with `Theme::set_theme`, and resolve them with `get_theme!`

Use `Preference::Reference(role.key())` to follow shared values and provide the expected type in every active theme

## Override individual theme fields

Use the generated builder methods for individual fields and keep the rest of the active theme

**CORRECT:** Set just the corner radius on this button

```rust
Button::new().filled().corner_radius(12.).child("Save")
```

**WRONG:** Build a partial theme by hand just to override one fixed value

```rust
Button::new()
    .filled()
    .theme_layout(ButtonLayoutThemePartial {
        corner_radius: Some(Preference::Specific(CornerRadius::new_all(12.))),
        ..Default::default()
    })
    .child("Save")
```

Pass a partial theme directly when sharing a group of overrides or when a field needs a `Preference::Reference` rather than a fixed value

## Downsample images for display

`ImageViewer` defaults to `DecodeMode::FromLayout` and uses the window scale factor when both dimensions are fixed in pixels

With flexible dimensions, use `DecodeMode::Custom` to cap the resulting raster size for thumbnails

The cap reduces the size of the retained raster, not the initial decode cost

**CORRECT:** Downsample a thumbnail with flexible width

```rust
ImageViewer::new(photo)
    .width(Size::fill())
    .height(Size::px(120.))
    .decode_mode(DecodeMode::Custom(Size2D::new(320., 240.)))
```

**WRONG:** Keep the full size raster for a thumbnail

```rust
ImageViewer::new(photo)
    .width(Size::fill())
    .height(Size::px(120.))
    .decode_mode(DecodeMode::Source)
```

## Choose an image sampling filter

Use `DecodeMode` for the resulting raster dimensions and `SamplingMode` for the scaling filter

Keep the default `Trilinear` for general use and try `Mitchell` for smoother scaling or `CatmullRom` for sharper scaling when the result needs it

**CORRECT:** Use `Nearest` for pixel art and keep smooth filtering for photos

```rust
rect()
    .child(ImageViewer::new(sprite).sampling_mode(SamplingMode::Nearest))
    .child(ImageViewer::new(photo))
```

**WRONG:** Force `Nearest` on a photo that needs smooth scaling

```rust
ImageViewer::new(photo).sampling_mode(SamplingMode::Nearest)
```

## Tune asset cache retention

`.asset_age(...)` controls how long an asset stays cached after it is no longer in use

Keep the default of one hour for assets you revisit often and shorten it when rarely revisited assets put pressure on memory

**CORRECT:** Set a shorter retention time for thumbnails in a large gallery that users rarely revisit

```rust
use std::time::Duration;

ImageViewer::new(photo).asset_age(Duration::from_secs(60))
```

**WRONG:** Keep every thumbnail cached until the app closes without a reason

```rust
ImageViewer::new(photo).asset_age(AssetAge::Unspecified)
```

## Rasterize complex SVGs in the background

Local SVGs rasterize synchronously by default and remote SVGs already rasterize in the background

Keep the default for cheap local icons and enable async rasterization for complex local artwork that could stall the UI

**CORRECT:** Rasterize complex local artwork in the background

```rust
SvgViewer::new(include_bytes!("./map.svg"))
    .width(Size::px(800.))
    .height(Size::px(600.))
    .async_rasterization(true)
```

**WRONG:** Leave expensive local artwork on the synchronous path when it stalls the UI

```rust
SvgViewer::new(include_bytes!("./map.svg"))
    .width(Size::px(800.))
    .height(Size::px(600.))
```

## Keep builder chains declarative

Use `.maybe` for a condition, `.map` for an `Option` value, and `.maybe_child` for an optional child

**CORRECT:** Group children behind one condition in the builder chain

```rust
rect().child("Status").maybe(show_details, |el| {
    el.child("Last synced today").child("No errors")
})
```

**WRONG:** Save the builder and reassign it to add children

```rust
let mut status = rect().child("Status");
if show_details {
    status = status.child("Last synced today").child("No errors");
}
status
```

## Pass plain text directly

Pass `&str` or `String` as a child when the text needs no custom styling or behavior

**CORRECT:** Pass strings directly

```rust
rect().child("Inbox").child(format!("Unread: {unread_count}"))
```

**WRONG:** Wrap every string in `label().text(...)`

```rust
rect()
    .child(label().text("Inbox"))
    .child(label().text(format!("Unread: {unread_count}")))
```

Use `label()` when the text needs its own styling or element properties

## Let layout adapt

**CORRECT:** Give the main pane the remaining width of a flex row

```rust
rect()
    .width(Size::fill())
    .horizontal()
    .content(Content::Flex)
    .child(rect().width(Size::px(220.)).child("Sidebar"))
    .child(rect().width(Size::flex(1.)).child("Content"))
```

**WRONG:** Hardcode the remaining width from one window size

```rust
rect()
    .width(Size::fill())
    .horizontal()
    .child(rect().width(Size::px(220.)).child("Sidebar"))
    .child(rect().width(Size::px(800.)).child("Content"))
```

## Call hooks in a stable order

**CORRECT:** Call hooks before branching and render optional children conditionally

```rust
let draft = use_state(String::new);

rect().maybe(show_editor, |el| el.child(Input::new(draft)))
```

**WRONG:** Call a hook inside the conditional builder callback

```rust
rect().maybe(show_editor, |el| {
    let draft = use_state(String::new);
    el.child(Input::new(draft))
})
```

If the draft should reset when the editor closes, put the hook in a conditionally mounted child component

## Derive values instead of syncing state

**CORRECT:** Compute cheap values directly from their source

```rust
let count = use_state(|| 0);

let doubled = *count.read() * 2;

label().text(doubled.to_string())
```

**WRONG:** Mirror a render value in a second state with an effect

```rust
let count = use_state(|| 0);
let mut doubled = use_state(|| 0);
use_side_effect(move || doubled.set(*count.read() * 2));

label().text(doubled.read().to_string())
```

## Cache expensive calculations with use_memo

Read reactive dependencies inside the memo and keep the callback free of side effects

**CORRECT:** Sort again only when the scores change

```rust
let ranked = use_memo(move || {
    let mut ranked = scores.read().clone();
    ranked.sort_unstable_by(|left, right| right.cmp(left));
    ranked
});

rect().children(ranked.read().iter().map(|score| score.to_string()))
```

**WRONG:** Sort a large list again on every unrelated render

```rust
let mut ranked = scores.read().clone();
ranked.sort_unstable_by(|left, right| right.cmp(left));

rect().children(ranked.iter().map(|score| score.to_string()))
```

## Derive values without PartialEq

`use_derived` computes the initial value immediately and recomputes in a task when dependencies change

It returns a `State<T>` and notifies subscribers on every recomputation

Prefer `use_memo` when the result implements `PartialEq` and unchanged results should not notify subscribers

**CORRECT:** Derive a preview whose type has no meaningful equality comparison

```rust
struct Preview {
    text: String,
}

let preview = use_derived(move || Preview {
    text: render_preview(&draft.read()),
});

rect().child(preview.read().text.clone())
```

**WRONG:** Use `use_memo` for the same `Preview` type even though it requires `PartialEq`

```rust
let preview = use_memo(move || Preview {
    text: render_preview(&draft.read()),
});

rect().child(preview.read().text.clone())
```

## Keep side effects out of render

Use `use_side_effect` to synchronize with something outside the view when the state it reads changes

Use an event handler instead when the work should happen only after that event

**CORRECT:** Update the window title when the document name changes

```rust
use_side_effect(move || {
    let title = document_name.read().clone();
    update_window_title(title);
});

rect().child(document_name.read().clone())
```

**WRONG:** Update the window title whenever the component renders

```rust
let title = document_name.read().clone();
update_window_title(title.clone());

rect().child(title)
```

## Animate changes in state

**CORRECT:** Let `use_animation_transition` animate when the target color changes

```rust
let color = use_animation_transition(target_color, |from, to| {
    AnimColor::new(from, to).time(250)
});

rect().background(&*color.read())
```

**WRONG:** Start the animation in `render` even when the target has not changed

```rust
let mut color = use_animation(move |_| {
    AnimColor::new(Color::TRANSPARENT, *target_color.read()).time(250)
});

color.start();

rect().background(&*color.read())
```

## Subscribe where values are displayed

Use `read()` for reactive rendering and `peek()` for snapshots that should not subscribe

Capture the `State` handle when a handler needs the latest value

Release state guards before calling another callback or awaiting a future, as shown in [Release state guards before awaiting](#release-state-guards-before-awaiting)

**CORRECT:** Read in the child that displays the value

```rust
fn render(&self) -> impl IntoElement {
    let count = *self.count.read();

    label().text(count.to_string())
}
```

**WRONG:** Subscribe the entire page just to pass the count to one badge

```rust
// In the page render
let count = *count.read();

rect().child(label().text(count.to_string()))
```

Keep state local with `use_state` and use context for shared dependencies

## Give children only the state access they need

Use `Readable<T>` for display, `Writable<T>` for direct mutation, and `WritableSetter<T>` when the parent controls how a replacement is applied

Convert at the call site or in the constructor so the child does not depend on how the parent stores the value

**CORRECT:** Give a display component a `Readable<T>` and turn a derived `State<T>` into a readable prop with `.into_readable()`

```rust
#[derive(PartialEq)]
struct TitleView {
    title: Readable<String>,
    key: DiffKey,
}

impl KeyExt for TitleView {
    fn key_mut(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for TitleView {
    fn render(&self) -> impl IntoElement {
        rect().child(self.title.read().clone())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

fn app() -> impl IntoElement {
    let document_name = use_state(|| "Draft".to_string());
    let title = use_derived(move || format!("Editing {}", document_name.read()));

    TitleView {
        title: title.into_readable(),
        key: DiffKey::None,
    }
}
```

**WRONG:** Give a display component write access it never uses

```rust
#[derive(PartialEq)]
struct TitleView {
    title: State<String>,
}
```

### Use Writable for direct edits from different sources

**CORRECT:** Let the same component edit either local state or a mutable Radio slice

```rust
#[derive(PartialEq)]
struct CountButton {
    count: Writable<usize>,
    key: DiffKey,
}

impl CountButton {
    fn new(count: impl IntoWritable<usize>) -> Self {
        Self {
            count: count.into_writable(),
            key: DiffKey::None,
        }
    }
}

impl KeyExt for CountButton {
    fn key_mut(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for CountButton {
    fn render(&self) -> impl IntoElement {
        let on_increment = {
            let mut count = self.count.clone();
            move |_: Event<PressEventData>| *count.write() += 1
        };

        let caption = format!("Count: {}", self.count.read());

        Button::new().on_press(on_increment).child(caption)
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

fn app() -> impl IntoElement {
    use_init_radio_station::<AppState, AppChannel>(AppState::default);
    let local_count = use_state(|| 0usize);
    let radio = use_radio::<AppState, AppChannel>(AppChannel::UnreadCount);

    let shared_count = radio.slice_mut_current(|state| &mut state.unread_count);

    rect()
        .child(CountButton::new(local_count))
        .child(CountButton::new(shared_count))
}
```

**WRONG:** Require `State<T>` in a reusable component even though it only needs to read and edit the value

```rust
#[derive(PartialEq)]
struct CountButton {
    count: State<usize>,
    key: DiffKey,
}
```

## Skip unchanged state writes

Use `set_if_modified` when an event can report the same value again and the value implements `PartialEq`

**CORRECT:** Notify subscribers only when the measured width changes

```rust
let mut width = use_state(|| 0.);

let on_sized = move |event: Event<SizedEventData>| {
    width.set_if_modified(event.area.width());
};

rect().width(Size::fill()).on_sized(on_sized)
```

**WRONG:** Write the width again when only the height changed

```rust
let mut width = use_state(|| 0.);

let on_sized = move |event: Event<SizedEventData>| {
    width.set(event.area.width());
};

rect().width(Size::fill()).on_sized(on_sized)
```

## Share dependencies through context

Provide shared clients or configuration once for the subtree that uses them

**CORRECT:** Provide a client at the app root and consume it where needed

```rust
fn app() -> impl IntoElement {
    use_provide_context(ApiClient::new);

    rect().child(AccountPage)
}

impl Component for AccountPage {
    fn render(&self) -> impl IntoElement {
        let client = use_consume::<ApiClient>();

        rect().child(client.service_name())
    }
}
```

**WRONG:** Construct a new client on every render of every consumer

```rust
impl Component for AccountPage {
    fn render(&self) -> impl IntoElement {
        let client = ApiClient::new();

        rect().child(client.service_name())
    }
}
```

## Scope shared state with Freya Radio

Enable the `radio` feature and initialize one station at the app root with `use_init_radio_station`

### Keep global app state in the existing station

If the app already uses Freya Radio, put new global app state in the existing station instead of providing a separate `State` through context

**CORRECT:** Add the unread count to the existing Radio model

```rust
#[derive(Default, Clone)]
struct AppState {
    profile: Profile,
    unread_count: usize,
}

use_init_radio_station::<AppState, AppChannel>(AppState::default);
```

**WRONG:** Create a parallel store through context while Radio already holds app state

```rust
use_init_radio_station::<AppState, AppChannel>(AppState::default);
use_provide_context(|| State::create(UnreadCount(0)));
```

Keep using context for shared dependencies such as API clients and configuration

### Subscribe by channel

**CORRECT:** Subscribe the badge to the unread count channel so profile edits do not notify it

```rust
let radio = use_radio::<AppState, AppChannel>(AppChannel::UnreadCount);

let unread_count = radio.read().unread_count;

label().text(format!("Unread: {unread_count}"))
```

**WRONG:** Subscribe to a catchall channel that every edit notifies

```rust
let radio = use_radio::<AppState, AppChannel>(AppChannel::All);

let unread_count = radio.read().unread_count;

label().text(format!("Unread: {unread_count}"))
```

Write through the channel that matches the change and use a `DataReducer` when several actions must enforce the same model rules

## Bind inputs directly

Use `WritableSetter::new` when edits must pass through a reducer

**CORRECT:** Pass writable state straight to `Input` and `Switch`

```rust
let name = use_state(String::new);
let enabled = use_state(|| true);

rect().child(Input::new(name)).child(Switch::new(enabled))
```

**WRONG:** Keep a second copy of the same input value and sync it with an effect

```rust
let mut name = use_state(String::new);
let edit = use_state(String::new);
use_side_effect(move || name.set(edit.read().clone()));

Input::new(edit)
```

## Key lists by item ID

**CORRECT:** Use a stable model ID unique among siblings

```rust
rect().children(items.iter().map(|item| {
    label().key(item.id).text(item.name.clone())
}))
```

**WRONG:** Key a list that can reorder by its current position

```rust
rect().children(items.iter().enumerate().map(|(index, item)| {
    label().key(index).text(item.name.clone())
}))
```

## Virtualize large lists

Keep ordinary children for small lists and virtualize when mounting every row is costly

**CORRECT:** Build only the rows needed for the viewport and keep stable item keys

```rust
let row_count = rows.len();

VirtualScrollView::new_with_data(rows, |item, rows| {
    let row = &rows[item.index];

    rect()
        .key(row.id)
        .height(Size::px(item.size))
        .child(row.title.clone())
        .into()
})
.length(row_count)
.item_size(32.)
.height(Size::fill())
```

**WRONG:** Mount every row in a regular scroll view for a large collection

```rust
ScrollView::new().height(Size::fill()).children(rows.iter().map(|row| {
    rect()
        .key(row.id)
        .height(Size::px(32.))
        .child(row.title.clone())
}))
```

## Load data outside render

Use `use_future` for a task on mount and `spawn` for async work started by a handler that updates Freya state

**CORRECT:** Render loading, success, and error from the same task

```rust
let profile = use_future(|| async { fetch_profile_name().await });

let caption = match &*profile.read() {
    FutureState::Pending | FutureState::Loading => "Loading".to_string(),
    FutureState::Fulfilled(Ok(name)) => name.clone(),
    FutureState::Fulfilled(Err(_)) => "Could not load profile".to_string(),
};

label().text(caption)
```

**WRONG:** Block `render` on a network request

```rust
let caption = fetch_profile_name_blocking();

label().text(caption)
```

## Show honest loading progress

Use `CircularLoader` when progress is unknown and there is no content layout to preserve

Use `Skeleton` when the shape of the content is known and `ProgressBar` when the operation reports a percentage from 0 to 100

Keep existing content visible during refresh and show a small loading indicator beside it

**CORRECT:** Show measured progress when available and a loader otherwise

```rust
rect()
    .child("Downloading")
    .map(progress_percent, |el, percent| el.child(ProgressBar::new(percent)))
    .maybe(progress_percent.is_none(), |el| el.child(CircularLoader::new()))
```

**WRONG:** Show a made up percentage while waiting for an operation that reports no progress

```rust
rect().child("Downloading").child(ProgressBar::new(50.))
```

**CORRECT:** Use a skeleton to reserve space for a profile title during the initial load

```rust
let title = if is_loading {
    Skeleton::new(true)
        .width(Size::fill())
        .height(Size::px(24.))
        .into_element()
} else {
    rect()
        .width(Size::fill())
        .height(Size::px(24.))
        .child(profile_name)
        .into_element()
};

rect().child(title)
```

Mount the skeleton only while loading so its animation stops when the real content appears

**WRONG:** Remove the title row while loading so the content below it shifts when the title appears

```rust
rect().maybe_child((!is_loading).then(|| {
    rect().height(Size::px(24.)).child(profile_name)
}))
```

## Spawn tasks from event handlers

`spawn` attaches work to the current scope and cancels it when that scope unmounts

Use `EventHandler::new_current` when work should follow the component that creates the handler rather than the button that receives the event

**CORRECT:** Start the refresh on press and tie it to the component that owns the status

```rust
let mut status = use_state(|| "Ready".to_string());

let on_refresh = EventHandler::new_current(move |_: Event<PressEventData>| {
    spawn(async move {
        status.set(fetch_status().await);
    });
});

rect()
    .child(Button::new().on_press(on_refresh).child("Refresh"))
    .child(status.read().clone())
```

**WRONG:** Spawn the request during render so its result can trigger another request

```rust
let mut status = use_state(|| "Ready".to_string());
spawn(async move {
    status.set(fetch_status().await);
});

rect().child(status.read().clone())
```

## Release state guards before awaiting

Never hold a guard from `.read()` or `.write()` across `.await`

Keep the `State<T>` handle, copy or clone the input before awaiting, and acquire a fresh guard only when you need it

**CORRECT:** Take an owned snapshot before the request and update state after it returns

```rust
spawn(async move {
    let user_name = profile.read().name.clone();
    let avatar = fetch_avatar(&user_name).await;
    profile.write().avatar = avatar;
});
```

**WRONG:** Keep a write guard alive while the request runs

```rust
spawn(async move {
    let mut profile = profile.write();
    profile.avatar = fetch_avatar(&profile.name).await;
});
```

## Build the Tokio runtime manually

Never use `#[tokio::main]` in a Freya app

If a dependency needs Tokio, build the runtime explicitly with `tokio::runtime::Builder` in a synchronous `main` and call `.enter()` before launching Freya

Keep the runtime and its guard alive until `launch` returns

Use Freya `spawn()` for tasks that update component state and reserve `tokio::spawn` for background work that does not touch it

**CORRECT:** Enter Tokio without wrapping Freya's event loop in an async `main`

```rust
fn main() -> std::io::Result<()> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    let _runtime_guard = runtime.enter();

    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
    Ok(())
}
```

**WRONG:** Launch Freya inside `#[tokio::main]`

```rust
#[tokio::main]
async fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}
```

## Share remote data with Freya Query

Enable the `query` feature and reuse the same query capability and key for the same resource

**CORRECT:** Use the same `FetchUserName` query in every view that needs this user

```rust
let user_name = use_query(Query::new(user_id, FetchUserName));

let caption = match &*user_name.read().state() {
    QueryStateData::Settled { res: Ok(name), .. }
    | QueryStateData::Loading { res: Some(Ok(name)) } => name.clone(),
    QueryStateData::Settled { res: Err(_), .. } => "Could not load user".to_string(),
    _ => "Loading".to_string(),
};

label().text(caption)
```

**WRONG:** Start a separate request for the same user in every view

```rust
let user_name = use_future(move || async move { fetch_user_name(user_id).await });

let caption = match &*user_name.read() {
    FutureState::Fulfilled(Ok(name)) => name.clone(),
    FutureState::Fulfilled(Err(_)) => "Could not load user".to_string(),
    _ => "Loading".to_string(),
};

label().text(caption)
```

After a successful mutation, call `QueriesStorage::<FetchUserName>::invalidate_matching(user_id).await` in `MutationCapability::on_settled`

## Keep cached data visible during refresh

**CORRECT:** Read successful data with `ok()` whether the query is settled or refreshing

```rust
let user_name = use_query(Query::new(user_id, FetchUserName));

let caption = {
    let query = user_name.read();
    let state = query.state();
    if let Some(name) = state.ok() {
        name.clone()
    } else if matches!(&*state, QueryStateData::Settled { res: Err(_), .. }) {
        "Could not load user".to_string()
    } else {
        "Loading".to_string()
    }
};

rect().child(caption)
```

**WRONG:** Hide a cached name whenever a refresh starts

```rust
let user_name = use_query(Query::new(user_id, FetchUserName));

let caption = match &*user_name.read().state() {
    QueryStateData::Settled { res: Ok(name), .. } => name.clone(),
    QueryStateData::Settled { res: Err(_), .. } => "Could not load user".to_string(),
    _ => "Loading".to_string(),
};

rect().child(caption)
```

## Use the router for page navigation

Enable the `router` feature for pages with paths or navigation history

Use `Link::new(Route::Settings)` for navigation controls and `RouterContext` when navigation follows an action

Keep local UI state for toggles and panels that are not separate pages

**CORRECT:** Declare routes and let Freya Router render the current page

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Home,
    #[route("/settings")]
    Settings,
}

fn app() -> impl IntoElement {
    Router::<Route>::new(RouterConfig::default)
}
```

**WRONG:** Track the current page in local state and match it manually

```rust
let mut page = use_state(|| Page::Home);

let on_settings = move |_: Event<PressEventData>| page.set(Page::Settings);

let content = match *page.read() {
    Page::Home => Home.into_element(),
    Page::Settings => Settings.into_element(),
};

rect()
    .child(Button::new().on_press(on_settings).child("Settings"))
    .child(content)
```

## Open external URLs through Freya

### Display a link

**CORRECT:** Use `Link` when the `router` feature is enabled and the control only navigates

```rust
Link::new("https://freyaui.dev").child("Freya docs")
```

**WRONG:** Wire a custom handler just to reproduce the same link

```rust
let on_open_docs = move |_: Event<PressEventData>| {
    Platform::get().open_url("https://freyaui.dev");
};

rect().on_press(on_open_docs).child("Freya docs")
```

### Open a URL from an action

**CORRECT:** Use `Platform::get().open_url(...)` in a custom handler or when the `router` feature is disabled

```rust
let on_open_docs = move |_: Event<PressEventData>| {
    close_help_menu();
    Platform::get().open_url("https://freyaui.dev");
};

Button::new().on_press(on_open_docs).child("Open docs")
```

**WRONG:** Add a separate opener dependency for the same action

```rust
let on_open_docs = move |_: Event<PressEventData>| {
    close_help_menu();
    let _ = open::that("https://freyaui.dev");
};

Button::new().on_press(on_open_docs).child("Open docs")
```

## Translate complete messages

**CORRECT:** Use a single Fluent message so translators can move the argument or change the punctuation

```ftl
welcome = Welcome, { $name }!
```

```rust
rect().child(t!("welcome", name: name))
```

**WRONG:** Put translated fragments together in a fixed order

```rust
rect().child(format!("{}, {name}!", t!("welcome_prefix")))
```

## Respect Android system bars and cutouts

Read `content_padding()` during render so edge to edge layouts follow changes to the system insets and window scale

**CORRECT:** Pad content with the current Android insets and use no extra padding on other platforms

```rust
#[cfg(target_os = "android")]
use freya::android::AndroidExt;

#[cfg(target_os = "android")]
let content_padding = Platform::get().content_padding();
#[cfg(not(target_os = "android"))]
let content_padding = Gaps::default();

rect().expanded().padding(content_padding).child("Settings")
```

**WRONG:** Guess a fixed status bar height and ignore the other edges

```rust
rect().expanded().padding((24., 0., 0., 0.)).child("Settings")
```

See the [Android example](../../examples/android) for setup with `AndroidPlugin`

## Test through the UI

Simulate the actual input with `freya-testing` and assert what the user sees

**CORRECT:** Click the control and check the updated label

```rust
let mut test = launch_test(counter_app);

test.click_cursor((15., 15.));

let updated_label = test.find(|_, element| {
    Label::try_downcast(element).filter(|label| label.text.as_ref() == "Count: 1")
});
assert!(updated_label.is_some());
```

**WRONG:** Set the counter directly to test that its button works

```rust
count.set(1);
assert_eq!(*count.peek(), 1);
```
