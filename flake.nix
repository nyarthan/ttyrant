{
  description = "morgensterm flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    treefmt-nix.url = "github:numtide/treefmt-nix";
    flake-root.url = "github:srid/flake-root";
  };

  outputs =
    inputs@{
      nixpkgs,
      flake-parts,
      treefmt-nix,
      flake-root,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        treefmt-nix.flakeModule
        flake-root.flakeModule
      ];

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];

      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        {
          treefmt.config = import ./treefmt.nix { inherit pkgs config; };

          devShells.default = pkgs.mkShell {
            packages =
              [
                pkgs.cargo
                pkgs.rustc
                pkgs.rustfmt
              ]
              ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
                pkgs.libiconv
                pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
                pkgs.darwin.apple_sdk.frameworks.ApplicationServices
                pkgs.darwin.apple_sdk.frameworks.CoreVideo
                pkgs.darwin.apple_sdk.frameworks.Carbon
                pkgs.darwin.apple_sdk.frameworks.AppKit
              ];
          };

          formatter = config.treefmt.build.wrapper;
        };
    };
}
