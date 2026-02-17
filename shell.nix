{ pkgs, ... }:
pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.rust-analyzer
  ];
}
