{ pkgs ? import ./rust_toolchain.nix }:

pkgs.mkShell {
  inputsFrom = with pkgs; [ rustc cargo ];
}
