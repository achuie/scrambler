{
  description = "Rubik's Cube scrambler";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, fenix, flake-compat }:
    utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      {
        defaultPackage = (pkgs.makeRustPlatform {
          inherit (fenix.packages.${system}.minimal) rustc cargo;
        }).buildRustPackage {
          pname = "scrambler";
          version = "0.1.0-alpha";
          src = ./.;
          cargoSha256 = "sha256-L70oVwjRil95xWrDa0ZxdqSyw9XAa5WSqs++nWT9eBo=";
        };
      }
    );
}
