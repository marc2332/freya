# Elements

Freya contains a set of primitive elements:

- [`label`](#label)
- [`image`](#image)
- [`svg`](#svg)
- [`paragraph and text`](#paragraph-and-text-and-text)

### label

The `label` element simply shows some text.

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        label {
            "Hello World"
        }
    )
}
```

### svg

The `svg` element let's you draw a SVG. You will need to use the `bytes_to_data` to transform the bytes into data the element can recognize.

Example:

```rust, no_run

static FERRIS: &[u8] = include_bytes!("./ferris.svg");

fn app(cx: Scope) -> Element {
    let ferris = bytes_to_data(cx, FERRIS);
    render!(
        svg {
            svg_data: ferris,
        }
    )
}
```

### image

The `image` element, just like `svg` element, require you to pass the image bytes yourself.

```rust, no_run
static RUST_LOGO: &[u8] = include_bytes!("./rust_logo.png");

fn app(cx: Scope) -> Element {
    let image_data = bytes_to_data(cx, RUST_LOGO);
    render!(
        image {
            image_data: image_data,
            width: "{size}",
            height: "{size}",
        }
    )
}
```

### paragraph and text

Both `paragraph` and `text` elements are used together. They will let you build texts with different styles.

``` rust
fn app(cx: Scope) -> Element {
    render!(
        paragraph {
            text {
                font_size: "15",
                "Hello, "
            }
            text {
                font_size: "30",
                "World!"
            }
        }
    )
}
```
