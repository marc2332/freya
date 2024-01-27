# Elements

Freya contains a set of primitive elements:

- [`rect`](#rect)
- [`label`](#label)
- [`image`](#image)
- [`svg`](#svg)
- [`paragraph and text`](#paragraph-and-text-and-text)

## rect

The `rect` element (aka `rectangle`) is a box where you can place as many elements inside you want.
You can specify things like [`width`](/guides/layout.html#width), [`paddings`](/guides/layout.html#padding) or even in what [`direction`](/guides/layout.html#direction) the inner elements are stacked.

Example:

```rust, no_run
fn app() -> Element {
    rsx!(
        rect {
            direction: "vertical",
            label { "Hi!" }
            label { "Hi again!"}
        }
    )
}
```

### label

The `label` element simply shows some text.

Example:

```rust, no_run
fn app() -> Element {
    rsx!(
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

fn app() -> Element {
    let ferris = bytes_to_data(FERRIS);
    rsx!(
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

fn app() -> Element {
    let image_data = bytes_to_data(RUST_LOGO);
    rsx!(
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
fn app() -> Element {
    rsx!(
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
