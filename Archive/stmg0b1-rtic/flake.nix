{
  description = "embassy flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
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
      in
      {
        devShells.default =
        let
          toolchain = pkgs.fenix.latest.toolchain;
          libv6 = pkgs.fenix.targets.thumbv6m-none-eabi.latest;
          libv7 = pkgs.fenix.targets.thumbv7em-none-eabihf.latest;
          rust = pkgs.fenix.combine [
            toolchain.rustc
            toolchain.rust-src
            toolchain.cargo
            toolchain.rustfmt
            toolchain.clippy
            libv6.rust-std
            libv7.rust-std
          ];
        in
        pkgs.mkShell {
          buildInputs = with pkgs; [
            rust

            # for flashing
            probe-rs-tools

            # for external deps
            pkg-config
          ];

					# set default defmt log level
					DEFMT_LOG = "info";
        };
      }
    );
}
