//! # Setup
//!
//! Make sure you have [Rust](https://www.rust-lang.org/) and your OS dependencies installed.
//!
//! ### Windows
//!
//! Install Visual Studio 2022 with the `Desktop Development with C++` workflow. 
//! You can learn learn more [here](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup#install-visual-studio-recommended-or-the-microsoft-c-build-tools).
//!
//! ### Linux
//!
//! #### Debian-based (Ubuntu, PopOS, etc)
//!
//! Install these packages:
//! ```sh
//! sudo apt install build-essential libssl-dev pkg-config cmake libgtk-3-dev libclang-dev
//! ```
//!
//! #### Arch Linux
//!
//! Install these packages:
//! ```sh
//! sudo pacman -S base-devel openssl cmake gtk3 clang
//! ```
//!
//! #### Fedora
//!
//! Install these packages:
//!
//! ```sh
//! sudo dnf install openssl-devel pkgconf cmake gtk3-devel clang-devel -y
//! sudo dnf groupinstall "Development Tools" "C Development Tools and Libraries" -y
//! ```
//!
//! #### NixOS
//!
//! Copy this [flake.nix](https://github.com/kuba375/freya-flake) into your project root. Then you can enter a dev shell by `nix develop`.
//!
//! Don't hesitate to contribute so other distros can be added here.
//!
//! ### MacOS
//!
//! No setup required. But feel free to add more if we miss something.
//!
//! ## Custom Linkers
//!
//! The following custom linkers are not supported at the moment:
//!
//! - `mold`
//!
//! If there is another one not supported don't hesitate to add it here.
