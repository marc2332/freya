{ inputs, pkgs, ... }:

let
  freyaLibraries = with pkgs; [
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
  freyaLibraryPath = pkgs.lib.makeLibraryPath freyaLibraries;
  nixGLPkgs = pkgs // {
    xorg = pkgs.xorg // {
      inherit (pkgs) libX11 libxcb libxshmfence;
    };
  };
  nixGL = pkgs.lib.callPackageWith nixGLPkgs (inputs.nixgl.outPath + "/nixGL.nix") { };
  rust = inputs.rust-overlay.lib.mkRustBin { } pkgs;
  stableRust = rust.stable."1.97.1".default.override {
    extensions = [ "rust-src" "rust-analyzer" ];
    targets = [ "wasm32-unknown-emscripten" ];
  };
  nightlyRust = rust.nightly."2026-03-15".minimal.override {
    extensions = [ "rust-src" "rust-analyzer" "rustfmt" ];
  };
in
{
  packages = with pkgs; [
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
    stableRust
    taplo
  ] ++ freyaLibraries;

  env = {
    LOCALE_ARCHIVE = "${pkgs.glibcLocales}/lib/locale/locale-archive";
    CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_LINKER = "${pkgs.emscripten}/bin/em++";
    CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_RUSTFLAGS = "-C link-arg=-sFULL_ES3=1";
    FREYA_NIGHTLY_CARGO = "${nightlyRust}/bin/cargo";
    FREYA_NIGHTLY_RUSTFMT = "${nightlyRust}/bin/rustfmt";
    MBX_DISPLAY = "plain";
    MBX_SAVINGS = "off";
    MBX_SUMMARY = "off";
  };

  scripts.gpu.exec = ''
    unset LD_LIBRARY_PATH
    exec ${nixGL.nixVulkanIntel}/bin/nixVulkanIntel \
      ${nixGL.nixGLIntel}/bin/nixGLIntel \
      ${pkgs.runtimeShell} -c '
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${freyaLibraryPath}"
        exec "$@"
      ' gpu "$@"
  '';

  scripts.mbx.exec = ''
    unset CARGO_TARGET_DIR NIX_LDFLAGS NIX_LDFLAGS_FOR_BUILD
    exec gpu "''${CARGO_HOME:-$HOME/.cargo}/bin/mbx" "$@"
  '';

  scripts.cargo.exec = ''
    unset CARGO_TARGET_DIR NIX_LDFLAGS NIX_LDFLAGS_FOR_BUILD
    MBX_CARGO_SHIM_MODE=1 MBX_CARGO_SHIM_PATH="$0" exec mbx "$@"
  '';

  enterShell = ''
    unset CARGO_TARGET_DIR

    mbx_root="''${CARGO_HOME:-$HOME/.cargo}"
    if [ ! -x "$mbx_root/bin/mbx" ]; then
      cargo binstall \
        --locked \
        --no-confirm \
        --root "$mbx_root" \
        mbx
    fi
  '';
}
