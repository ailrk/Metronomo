{
  description = "Metronomo";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/af85bf3cb30388361a607717a0a55c2050225069";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
  flake-utils.lib.eachDefaultSystem (system:
  let
    pkgs = import nixpkgs {
      system = system;
    };
  in {
    packages.default = pkgs.rustPlatform.buildRustPackage {
      pname = "metronomo";
      version = "0.1.0";

      src = ./.;

      cargoLock = {
        lockFile = ./Cargo.lock;
      };
    };

    devShell = pkgs.callPackage ./shell.nix {};
  }
  );
}
