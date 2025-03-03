//! ### Color syntax
//!
//! The attributes that have colors as values can use the following syntax:
//!
//! #### Static colors
//! - `red`
//! - `blue`
//! - `green`
//! - `yellow`
//! - `black` (default for `color` attribute)
//! - `gray`
//! - `white` (default for `background` attribute)
//! - `orange`
//! - `transparent`
//!
//! #### rgb() / hsl() / Hex
//!
//! - With RGB: `rgb(150, 60, 20)`
//! - With RGB and alpha: `rgb(150, 60, 20, 0.7)`
//!     - You can also use 0-255 for the alpha: `rgb(150, 60, 20, 70)`
//! - With HSL: `hsl(28deg, 80%, 50%)`
//! - With HSL and alpha: `hsl(28deg, 80%, 50%, 25%)`
//! - With Hex: `#E93323`
//!
//! #### Gradients
//!
//!  Syntax is `<type>-gradient(<angle>, ...[<color> <offset>],)`
//!  And supported types are `linear`/`radial`/`conic`
//!
//!  Examples:
//!     Linear: `linear-gradient(250deg, orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)`
//!     Radial: `radial-gradient(orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)`
//!     Conic: `conic-gradient(250deg, orange 15%, rgb(255, 0, 0) 50%, rgb(255, 192, 203) 80%)`
//!
//! #### SVG
//!
//! For the `svg` element you can also use:
//!
//! - `current_color`: Use the inherited color from the `color` attribute.
