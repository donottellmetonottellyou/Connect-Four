# Helped by https://github.com/jraygauthier/jrg-rust-cross-experiment
{ pkgs ? import <nixpkgs> { } }:
let
  rustToolchain = "stable";
  rustTargetWin = "x86_64-pc-windows-gnu";

  pkgs-cross-mingw = import pkgs.path {
    crossSystem = {
      config = "x86_64-w64-mingw32";
    };
  };
  mingw_w64_cc = pkgs-cross-mingw.stdenv.cc;
  mingw_w64 = pkgs-cross-mingw.windows.mingw_w64;
  mingw_w64_pthreads = pkgs-cross-mingw.windows.mingw_w64_pthreads;

  wine = pkgs.wineWowPackages.stable;
in
pkgs.mkShell {
  buildInputs = [
    pkgs.godot_4

    pkgs.rustup
    mingw_w64_cc

    wine

    (pkgs.writeShellScriptBin "cargo-build-all" ''
      cargo build --release && cargo build --release --target=${rustTargetWin}
    '')
    (pkgs.writeShellScriptBin "cargo-test-all" ''
      cargo test && cargo test --target=${rustTargetWin}
    '')
  ];

  WINEPREFIX = toString ./.wine;
  RUSTUP_TOOLCHAIN = rustToolchain;
  CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUNNER = "${wine}/bin/wine64";
  CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
    mingw_w64
    mingw_w64_pthreads
  ]);

  shellHook = ''
    rustup install --profile default stable > /dev/null
    rustup override set ${rustToolchain} > /dev/null
    rustup target add "${rustTargetWin}" > /dev/null
    rustup show
    echo "godot $(godot4 --version)"
    echo "run cargo-build-all for releases, cargo-test-all for testing"
  '';
}
