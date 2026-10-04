{
  description = "Freya development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixgl = {
      url = "github:nix-community/nixGL";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    nixgl,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    libraries = with pkgs; [
      atk
      cairo
      fontconfig
      freetype
      gdk-pixbuf
      glib
      gtk3
      libGL
      libclang
      libgcc.lib
      libx11
      libxkbcommon
      libxcursor
      libxi
      libxrandr
      libsoup_3
      llvmPackages.bintools
      openssl
      pkg-config
      udev
      vulkan-loader
      wayland
      webkitgtk_4_1
      xdo
      xdotool
    ];
    libraryPath = pkgs.lib.makeLibraryPath libraries;
    nixGLPkgs =
      pkgs
      // {
        xorg =
          pkgs.xorg
          // {
            inherit (pkgs) libX11 libxcb libxshmfence;
          };
      };
    nixGL = pkgs.lib.callPackageWith nixGLPkgs (nixgl.outPath + "/nixGL.nix") {};
    rust = rust-overlay.lib.mkRustBin {} pkgs;
    stableRust = rust.stable."1.97.1".default.override {
      extensions = ["rust-src" "rust-analyzer"];
      targets = ["wasm32-unknown-emscripten"];
    };
    androidRust = stableRust.override {
      targets = [
        "wasm32-unknown-emscripten"
        "aarch64-linux-android"
        "x86_64-linux-android"
      ];
    };
    androidPkgs = import nixpkgs {
      inherit system;
      config = {
        allowUnfree = true;
        android_sdk.accept_license = true;
      };
    };
    androidSdk =
      (androidPkgs.androidenv.composeAndroidPackages {
        platformVersions = ["36" "36.1"];
        buildToolsVersions = ["36.0.0"];
        includeNDK = true;
        ndkVersions = ["26.3.11579264"];
        includeCmake = false;
      }).androidsdk;
    androidSdkPath = "${androidSdk}/libexec/android-sdk";
    androidNdkPath = "${androidSdkPath}/ndk/26.3.11579264";
    nightlyRust = rust.nightly."2026-03-15".minimal.override {
      extensions = ["rust-src" "rust-analyzer" "rustfmt"];
    };
    gpu = pkgs.writeShellScriptBin "gpu" ''
      unset LD_LIBRARY_PATH
      exec ${nixGL.nixVulkanIntel}/bin/nixVulkanIntel \
        ${nixGL.nixGLIntel}/bin/nixGLIntel \
        ${pkgs.runtimeShell} -c '
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${libraryPath}"
          exec "$@"
        ' gpu "$@"
    '';
    mbx = pkgs.writeShellScriptBin "mbx" ''
      unset CARGO_TARGET_DIR NIX_LDFLAGS NIX_LDFLAGS_FOR_BUILD
      exec ${gpu}/bin/gpu "''${CARGO_HOME:-$HOME/.cargo}/bin/mbx" "$@"
    '';
    cargo = pkgs.writeShellScriptBin "cargo" ''
      unset CARGO_TARGET_DIR NIX_LDFLAGS NIX_LDFLAGS_FOR_BUILD
      MBX_CARGO_SHIM_MODE=1 MBX_CARGO_SHIM_PATH="$0" exec ${mbx}/bin/mbx "$@"
    '';
    mkDevShell = rustToolchain: extraPackages:
      pkgs.mkShell {
        packages = with pkgs;
          [
            cargo
            mbx
            gpu
            rustToolchain
            alejandra
            cargo-binstall
            cargo-nextest
            clang
            cmake
            dioxus-cli
            emscripten
            git
            curl
            just
            python3
            taplo
          ]
          ++ libraries
          ++ extraPackages;

        LOCALE_ARCHIVE = "${pkgs.glibcLocales}/lib/locale/locale-archive";
        CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_LINKER = "${pkgs.emscripten}/bin/em++";
        CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_RUSTFLAGS = "-C link-arg=-sFULL_ES3=1";
        FREYA_NIGHTLY_CARGO = "${nightlyRust}/bin/cargo";
        FREYA_NIGHTLY_RUSTFMT = "${nightlyRust}/bin/rustfmt";

        shellHook = ''
          unset CARGO_TARGET_DIR

          mbx_root="''${CARGO_HOME:-$HOME/.cargo}"
          if [ ! -x "$mbx_root/bin/mbx" ]; then
            PATH="${rustToolchain}/bin:$PATH" ${pkgs.cargo-binstall}/bin/cargo-binstall \
              --locked --no-confirm --root "$mbx_root" mbx
          fi

          export PATH="${cargo}/bin:${mbx}/bin:${gpu}/bin:$PATH"
        '';
      };
  in {
    formatter.${system} = pkgs.alejandra;
    devShells.${system} = {
      default = mkDevShell stableRust [];
      android = (mkDevShell androidRust [androidSdk pkgs.cargo-ndk pkgs.jdk17]).overrideAttrs (_: {
        ANDROID_HOME = androidSdkPath;
        ANDROID_SDK_ROOT = androidSdkPath;
        ANDROID_NDK_HOME = androidNdkPath;
        ANDROID_NDK = androidNdkPath;
        JAVA_HOME = "${pkgs.jdk17}";
        GRADLE_OPTS = "-Dorg.gradle.project.android.aapt2FromMavenOverride=${androidSdkPath}/build-tools/36.0.0/aapt2";
      });
    };
  };
}
