let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    nodejs
  ];
}
