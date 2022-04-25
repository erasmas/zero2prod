{ pkgs ? import <nixpkgs> {
  overlays = [
    (import (builtins.fetchTarball
      "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
  ];
} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    openssl
    rust-bin.stable.latest.default
    # rust-bin.nightly.latest.default
    clippy
    postgresql
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
