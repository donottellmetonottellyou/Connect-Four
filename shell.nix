# Helped by https://github.com/jraygauthier/jrg-rust-cross-experiment
{ pkgs ? import <nixpkgs> { } }:
let
  cwd = builtins.toString ./.;

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

    (pkgs.writeShellScriptBin "open-editor" ''
      cd ${cwd}/godot
      nohup godot4 -e --path . > /dev/null &
    '')
    (pkgs.writeShellScriptBin "build-dev" ''
      cd ${cwd}/rust
      # Clean release for export sanity
      cargo clean --profile release
      cargo build && cargo build --target=${rustTargetWin}
    '')
    (pkgs.writeShellScriptBin "build-test" ''
      cd ${cwd}/rust
      # Clean release for export sanity
      cargo clean --profile release
      cargo test && cargo test --target=${rustTargetWin}
    '')
    (pkgs.writeShellScriptBin "build-release" ''
      cd ${cwd}/rust
      # Run cargo test instead of build for sanity check
      cargo test --release && cargo test --release --target=${rustTargetWin}
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
    echo
    echo "Commands: open-editor, build-dev, build-test, build-release"
  '';
}
