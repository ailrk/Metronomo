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

      nativeBuildInputs = [
        pkgs.wasm-bindgen-cli
        pkgs.llvmPackages.bintools
      ];

      buildPhase = ''
        cargo build --target wasm32-unknown-unknown --release --offline
        wasm-bindgen --target web --out-dir dist \
          target/wasm32-unknown-unknown/release/metronomo.wasm
      '';

      installPhase = ''
        mkdir -p $out
        cp index.html $out/
        [ -f index.css ] && cp index.css $out/
        cp -r dist/* $out/
      '';

      cargoLock = {
        lockFile = ./Cargo.lock;
      };
    };

    nixosModules = rec {
      metronomo = import ./nix/modules self;
      default = metronomo;
    };

    devShell = pkgs.callPackage ./shell.nix {};
  }
  );
}
