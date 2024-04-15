//! ### Color syntax
//!
//! The attributes that have colors as values can use the following syntax:
//!
//! #### Static colors
//! - `rect`
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
