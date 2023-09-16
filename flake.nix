{
  description = "Rubik's Cube scrambler";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix }:
    let
      forAllSystems = f: nixpkgs.lib.genAttrs [ "x86_64-linux" ] (system:
        f { pkgs = nixpkgs.legacyPackages.${system}; fenix-pkgs = fenix.packages.${system}; }
      );
    in
    {
      packages = forAllSystems (pset: with pset;
        let
          src = nixpkgs.lib.sourceByRegex self [ "src(/.*)?" "Cargo\\.(toml|lock)" ];
          toolchain = fenix-pkgs.fromToolchainFile {
            file = ./rust-toolchain.toml;
            # Generate by `nix build`ing with `nixpkbs.lib.fakeSha256.
            sha256 = "sha256-Q9UgzzvxLi4x9aWUJTn+/5EXekC98ODRU1TwhUs9RnY=";
          };
        in
        {
          default = (pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          }).buildRustPackage {
            pname = "scrambler";
            inherit ((nixpkgs.lib.importTOML "${src}/Cargo.toml").package) version;

            inherit src;
            cargoLock.lockFile = "${src}/Cargo.lock";
          };
        });

      formatter = forAllSystems (pset: pset.pkgs.nixpkgs-fmt);
    };
}
