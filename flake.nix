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
        pkgs = import nixpkgs { inherit system; };

        commonPackages = with pkgs; [
          python3
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

          # WINIT_UNIX_BACKEND=wayland
          wayland

          # WINIT_UNIX_BACKEND=x11
          libxcursor
          libxrandr
          libxi
          libx11
        ];

        mkDevShell =
          toolchains:
          pkgs.mkShell rec {
            packages = commonPackages ++ toolchains;
            buildInputs = commonBuildInputs;
            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
          };
      in
      {
        devShells = {
          default = mkDevShell [ ];
          stable = mkDevShell [ fenix.packages.${system}.stable.toolchain ];
          unstable = mkDevShell [ fenix.packages.${system}.default.toolchain ];
        };
      }
    )
    // {
      templates.default = {
        path = ./nix-template;
        description = "Basic nix template to set up a devshell";
      };
    };
}
