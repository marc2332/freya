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

        commonPackages = [
          pkgs.python3
          pkgs.dioxus-cli
          pkgs.taplo
        ];

        commonBuildInputs = with pkgs; [
          libxkbcommon
          libGL
          udev
          openssl
          pkg-config
          fontconfig
          libgcc.lib
          freetype
          gtk3
          glib
          cairo
          gdk-pixbuf
          libsoup_3
          pango
          atk
          webkitgtk_4_1
          xdo
          xdotool

          llvmPackages.bintools

          # WINIT_UNIX_BACKEND=wayland
          wayland

          # WINIT_UNIX_BACKEND=x11
          libxcursor
          libxrandr
          libxi
          libx11
        ];

        mkDevShell =
          toolchain:
          pkgs.mkShell rec {
            packages = commonPackages ++ [
              toolchain
            ];
            buildInputs = commonBuildInputs;
            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
          };
      in
      {
        devShells = {
          default = mkDevShell fenix.packages.${system}.stable.toolchain;
          unstable = mkDevShell fenix.packages.${system}.default.toolchain;
        };
      }
    );
}
