{
  description = "Rubik's Cube scrambler";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgsFor.${system};
          src = pkgs.lib.sourceByRegex self [ "src(/.*)?" "Cargo\\.(toml|lock)" ];
        in
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "scrambler";
            inherit ((pkgs.lib.importTOML "${src}/Cargo.toml").package) version;

            inherit src;
            cargoLock = { lockFile = "${src}/Cargo.lock"; };
          };
        });
      formatter = forAllSystems (system: nixpkgsFor.${system}.nixpkgs-fmt);
    };
}
