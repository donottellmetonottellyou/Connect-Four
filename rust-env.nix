{ pkgs ? import <nixpkgs> { } }: {
  buildInputs = with pkgs; [
    rustup
  ];

  shellHook = ''
    rustup install --no-self-update --profile default stable > /dev/null
    rustup override set stable > /dev/null
    rustc --version
  '';
}
