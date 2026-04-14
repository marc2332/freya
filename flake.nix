{
  description = "A development environment for compiling anything using Freya";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.systems.url = "github:nix-systems/default";
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.systems.follows = "systems";
  };
  inputs.fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    fenix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};

        commonPackages = [
          pkgs.python3
          pkgs.just
          pkgs.taplo
          pkgs.alejandra
        ];

        commonBuildInputs = [
          pkgs.libxkbcommon
          pkgs.libGL
          pkgs.udev
          pkgs.openssl
          pkgs.pkg-config
          pkgs.fontconfig
          pkgs.libgcc.lib
          pkgs.freetype

          # required by "webview" and "tray" `--features`
          pkgs.glib
          pkgs.gtk3
          pkgs.webkitgtk_4_1
          pkgs.libsoup_3
          pkgs.xdotool

          # WINIT_UNIX_BACKEND=wayland
          pkgs.wayland

          # WINIT_UNIX_BACKEND=x11
          pkgs.libxcursor
          pkgs.libxrandr
          pkgs.libxi
          pkgs.libx11
        ];

        mkDevShell = toolchain:
          pkgs.mkShell rec {
            packages =
              commonPackages
              ++ [
                toolchain
              ];
            buildInputs = commonBuildInputs;
            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
          };

        stableToolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          # sha256 = pkgs.lib.fakeSha256;
          sha256 = "sha256-zC8E38iDVJ1oPIzCqTk/Ujo9+9kx9dXq7wAwPMpkpg0=";
        };

        nightlyToolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain-nightly.toml;
          # sha256 = pkgs.lib.fakeSha256;
          sha256 = "sha256-4ot8+Fs79G1hUwlEYI6700QBLKdkLb33yzwOou1o5Yk=";
        };
      in {
        formatter = pkgs.alejandra;

        devShells = {
          default = mkDevShell stableToolchain;
          unstable = mkDevShell nightlyToolchain;
        };
      }
    );
}
