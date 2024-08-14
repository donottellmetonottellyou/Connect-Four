{ pkgs ? import <nixpkgs> { } }:
let
  rustEnv = import ./rust-env.nix { inherit pkgs; };
  godotEnv = import ./godot-env.nix { inherit pkgs; };
in
pkgs.mkShell {
  buildInputs = [ ] ++ rustEnv.buildInputs ++ godotEnv.buildInputs;
  shellHook = '''' + rustEnv.shellHook + godotEnv.shellHook;
}
