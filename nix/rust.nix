{ pkgs, gitignore }:

let
  rustVersion = (pkgs.rust-bin.fromRustupToolchainFile
    ../rust-toolchain.toml); # rust-bin.stable.latest.default
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rustVersion;
    rustc = rustVersion;
  };
in {
  rust-shell =
    (rustVersion.override { extensions = [ "rust-src" "rust-analyzer" ]; });

}
