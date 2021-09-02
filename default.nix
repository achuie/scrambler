let
  mozilla-pkgs = import (builtins.fetchTarball
    https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> {
    overlays = [
      mozilla-pkgs (self: super:
      {
        rustc = (self.rustChannelOf { rustToolchain = ./rust-toolchain; }).rust;
        cargo = (self.rustChannelOf { rustToolchain = ./rust-toolchain; }).rust;
      })
    ];
  };
  naersk = pkgs.callPackage (builtins.fetchGit {
    name = "master";
    url = "https://github.com/nix-community/naersk";
    ref = "HEAD";
    rev = "df71f5e4babda41cd919a8684b72218e2e809fa9";
  }) {};
in naersk.buildPackage ./.
