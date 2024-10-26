{ pkgs, config, ... }:
{
  inherit (config.flake-root) projectRootFile;
  package = pkgs.treefmt;
  settings = {
    global.excludes = [
      ".direnv"
      ".envrc"
    ];
  };
  programs = {
    nixfmt.enable = true;
    mdformat.enable = true;
    taplo.enable = true;
    rustfmt.enable = true;
  };

}
