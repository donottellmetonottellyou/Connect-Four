{ pkgs ? import <nixpkgs> { } }: {
  buildInputs = with pkgs; [
    godot_4
  ];

  shellHook = ''
    echo "godot $(godot4 --version)"
  '';
}
