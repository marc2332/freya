# Style

Learn how the style attributes work.

- [`background`](#background)
- [`shadow`](#shadow)
- [`radius`](#radius)
- [`border`](#border)

### background

The `background` attribute will let you specify a color as the background of the element.

You can learn about the syntax of this attribute [here](#color-syntax).

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            background: "red"
        }
    )
}
```

Compatible elements: [`rect`](/guides/elements.html#rect)


### shadow

The `shadow` attribute let's you draw a shadow outside of the element.

Syntax: `<x> <y> <intensity> <size> <color>`

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            shadow: "0 0 25 2 rgb(0, 0, 0, 120)"
        }
    )
}
```

Compatible elements: [`rect`](/guides/elements.html#rect)

### radius

The `radius` attribute let's you smooth the corners of the element.

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            radius: "10"
        }
    )
}
```

Compatible elements: [`rect`](/guides/elements.html#rect)

### border

You can add a border to an element using the `border` and `border_alignment` attributes.

- `border` syntax: `[width] <inner | outer | center> [color]`.
- `border_alignment` syntax: `<none | solid>`.

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            border: "2 center black",
            border_alignment: "inner"
        }
    )
}
```

Compatible elements: [`rect`](/guides/elements.html#rect)

### overflow

Specify how overflow should be handled.

Accepted values: `clip | none`.

Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            overflow: "clip"
            width: "100",
            height: "100%",
            rect {
                width: "500",
                height: "100%",
                background: "red",
            }
        }
    )
}
```

Compatible elements: [`rect`](/guides/elements.html#rect)

### Color syntax

The attributes that have colors as values can use the following syntax:

#### Static colors
- `rect`
- `blue`
- `green`
- `yellow`
- `black` (default for `color` attribute)
- `gray`
- `white` (default for `background` attribute)
- `orange`
- `transparent`

#### rgb() / hsl()

- With RGB: `rgb(150, 60, 20)`
- With RGB and alpha: `rgb(150, 60, 20, 70)`
- With HSL: `hsl(28, 0.8, 0.5)`
- With HSL and alpha: `hsl(28, 0.8, 0.5, 0.25)`

### Inheritance

These are some attribute that are inherited from the element parents:

- `color`
- `font_family`
- `font_size`
- `font_style`
- `font_weight`
- `font_width`
- `line_height`
- `align`
- `max_lines`
