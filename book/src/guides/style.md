# Style

Learn how the style attributes work.

- [`background`](#background)
- [`color`](#color)
- [`shadow`](#shadow)
- [`radius`](#radius)
- [`font_family`](#font_family)
- [`font_size`](#font_size)
- [`align`](#align)
- [`max_lines`](#max_lines)
- [`font_style`](#font_style)
- [`font_weight`](#font_weight)
- [`font_width`](#font_width)
- [`border`](#border)

### background

The `background` attribute will let you specify a color as the background of the element.

You can learn about the syntax of this attribute [here](#color-syntax).

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        rect {
            background: "red"
        }
    )
}
```

Compatible elements: [`rect`](/guides/elements.html#rect), [`container`](/guides/elements.html#container)

### color

The `color` attribute let's you specify the color of the text.

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            color: "green",
            "Hello, World!"
        }
    )
}
```

Another example showing [inheritance](#inheritance):

```rust
fn app(cx: Scope) -> Element {
    render!(
        rect {
            color: "blue",
            label {
                "Hello, World!"
            }
        }
    )
}

```

Compatible elements: [`label`](/guides/element.html#label), [`paragraph`](/guides/elements.html#paragraph), [`text`](/guides/elements.html#text)


### shadow

The `shadow` attribute let's you draw a shadow outside of the element.

Syntax: `<x> <y> <intensity> <size> <color>`

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        rect {
            shadow: "0 0 50 10 black"
        }
    )
}
```

Compatible elements: [`rect`](/guides/elements.html#rect), [`container`](/guides/elements.html#container)

### radius

The `radius` attribute let's you smooth the corners of the element.

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        rect {
            radius: "10"
        }
    )
}
```

Compatible elements: [`rect`](/guides/elements.html#rect), [`container`](/guides/elements.html#container)

### font_family

With the `font_family` you can specify what font do you want to use for the inner text.

Limitation: Only fonts installed in the system are supported for now.

Example: 

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_family: "Inter",
            "Hello, World!"
        }
    )
}
```

Compatible elements: [`label`](/guides/element.html#label), [`paragraph`](/guides/elements.html#paragraph), 

### font_size

You can specify the size of the text using `font_size`.

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_size: "50",
            "Hellooooo!"
        }
    )
}
```

Compatible elements: [`label`](/guides/element.html#label), [`paragraph`](/guides/elements.html#paragraph), [`text`](/guides/elements.html#text)

### align

You can change the alignment of the text using the `align` attribute.

Accepted values: `center`, `end`, `justify`, `left`, `right`, `start`

Example

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            align: "right",
            "Hello, World!"
        }
    )
}
```

Compatible elements: [`label`](/guides/element.html#label), [`paragraph`](/guides/elements.html#paragraph), 

### max_lines

You can limit the amount of shown lines with the `max_lines` attribute.

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            max_lines: "1",
            "Hello, World! \n Hello, again!"
        }
    )
}
```

Compatible elements: [`label`](/guides/element.html#label), [`paragraph`](/guides/elements.html#paragraph), 


### font_style

You can choose a style for a text using the `font_style` attribute.

Accepted values: `upright` (default), `italic` and `oblique`.

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_style: "italic",
            "Hello, World!"
        }
    )
}
```

### font_weight

You can choose a weight for a text using the `font_weight` attribute.

Accepted values:
- `invisible`
- `thin`
- `extra-light`
- `light`
- `normal` (default)
- `medium`
- `semi-bold`
- `bold`
- `extra-bold`
- `black`
- `extra-black`
- `50`
- `100`
- `200`
- `300`
- `400`
- `500`
- `600`
- `700`
- `800`
- `900`
- `950`

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_weight: "bold",
            "Hello, World!"
        }
    )
}
```

### font_width

You can choose a width for a text using the `font_width` attribute.

Accepted values:
- `ultra-condensed`
- `extra-condensed`
- `condensed`
- `normal` (default)
- `semi-expanded`
- `expanded`
- `extra-expanded`
- `ultra-expanded`

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        label {
            font_weight: "bold",
            "Hello, World!"
        }
    )
}
```

### border

You can add a border to an element using the `border` and `border_alignment` attributes.

- `border` syntax: `[width] <inner | outer | center> [color]`.
- `border_alignment` syntax: `<none | solid>`.

Example:

```rust
fn app(cx: Scope) -> Element {
    render!(
        rect {
            border: "2 center black",
            border_alignment: "inner"
        }
    )
}
```

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

#### rgb() / rgba()

- With RGB: `rgb(150, 60, 20)`
- With RGBA: `rgba(30, 50, 200, 70)`

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
