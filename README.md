# Metronomo

A Metronome app.


<img src="doc/1.png" width=400>

## Nix

This project provides a NixOS module to host the it as a static web service.

To deploy with nix, add this repo to your `flake.nix` inputs:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    # Point to the repository (or a local path for development)
    metronomo.url = "github:ailrk/metronomo";
  };

  outputs = { self, nixpkgs, metronomo, ... }: {
    nixosConfigurations.my-server = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        metronomo.nixosModules.default
        ./configuration.nix
      ];
    };
  };
}
```

Then you can enable the servce in `configuration.nix`. This will setup an internal nginx instance to serve the wasm and html files.

```nix
{
  services.metronomo.metronome = {
    enable = true;
    port = 8004; # The internal port to serve on
  };
}
```

If you need public access, just proxy it with a public facing nginx:

```nix
{
  services.nginx.virtualHosts."metronome.example.com" = {
    forceSSL = true;
    enableACME = true;
    locations."/" = {
      proxyPass = "http://localhost:8004";
    };
  };
}
```
