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
in pkgs
