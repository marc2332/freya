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

  outputs =
    {
      nixpkgs,
      flake-utils,
      fenix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        formatter = pkgs.alejandra;

        devShells = {
          default = pkgs.mkShell rec {
            packages = [
              pkgs.python3
              pkgs.just
              pkgs.taplo
              pkgs.cargo-nextest
              pkgs.alejandra
              pkgs.dioxus-cli
            ];
            buildInputs = with pkgs; [
              libxkbcommon
              libGL
              udev
              openssl
              pkg-config
              fontconfig
              libgcc.lib
              freetype
              cairo
              gdk-pixbuf
              pango
              atk
              xdo

              llvmPackages.bintools

              # required by "webview" and "tray" `--features`
              glib
              gtk3
              webkitgtk_4_1
              libsoup_3
              xdotool

              # WINIT_UNIX_BACKEND=wayland
              wayland

              # WINIT_UNIX_BACKEND=x11
              libxcursor
              libxrandr
              libxi
              libx11
            ];
            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
          };
        };
      }
    );
}
