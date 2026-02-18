self: { config, lib, pkgs, ... }:
let
  cfgs = config.services.metronomo;
in
{
  options.services.metronomo =
    let submodule = { name, ... }:
      {
        options = {
          enable = lib.mkOption {
            type = lib.types.bool;
            default = false;
            description = "Enable the Metronomo static web app.";
          };

          hostName = lib.mkOption {
            type = lib.types.string;
            example = "metronome.example.com";
            default = "${name}.internal";
            description = "The hostname to serve the metronome on.";
          };

          port = lib.mkOption {
            type = lib.types.port;
            default = 8080;
            description = "Port for the local Nginx virtual host.";
          };

          package = lib.mkOption {
            type = lib.types.package;
            default = self.packages.${pkgs.system}.default;
            description = "The metronomo package containing static files (index.html, pkg/, etc).";
          };
        };
      };
    in
    lib.mkOption {
      description = "Configure metronomo instances.";
      default = { };
      type = lib.types.attrsOf (lib.types.submodule submodule);
    };

  # setup an internal
  config = lib.mkIf (lib.any (cfg: cfg.enable) (lib.attrValues cfgs)) {
    services.nginx = {
      enable = true;
      virtualHosts = lib.mapAttrs (name: cfg: {
        listen = [{ addr = "0.0.0.0"; port = cfg.port; }];
        serverName = cfg.hostName;
        locations."/" = {
          root = "${cfg.package}";
          index = "index.html";
          extraConfig = ''
            include ${pkgs.nginx}/conf/mime.types;
            types {
              application/wasm wasm;
            }
            default_type text/html; # Fallback to HTML
          '';
        };
      }) cfgs;
    };
  };
}
