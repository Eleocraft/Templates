{
  description = "devshell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } ({ ... }: {
      systems = import inputs.systems;
      perSystem = { pkgs, ... }: 
      {
        devShells.default =
        pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
          {% for dep in dependencies | split(pat=",") %}  {{ dep | trim }}
          {% endfor %}];
        };
      };
    });
}
