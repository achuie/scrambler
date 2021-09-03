let
  pkgs = import ./rust_toolchain.nix;
  naersk = pkgs.callPackage (builtins.fetchGit {
    name = "master";
    url = "https://github.com/nix-community/naersk";
    ref = "HEAD";
    rev = "df71f5e4babda41cd919a8684b72218e2e809fa9";
  }) {};
in naersk.buildPackage ./.
