{
  description = "rust flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };
        rust-toolchain = pkgs.fenix.{{ rust_version }}.toolchain;
      in
      {
        devShells.default =
        pkgs.mkShell {
          buildInputs = [
            rust-toolchain
            {% if rust_version == "stable" %}
            rust-analyzer
            {% else %}
            rust-analyzer-nightly
            {% endif %}
          ];

          # set the rust src for rust_analyzer
          RUST_SRC_PATH = "${rust-toolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
