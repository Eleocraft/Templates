{
  description = "Notebook development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } ({ ... }: {
      systems = import inputs.systems;
      perSystem = { pkgs, ... }: 
      let
        python = pkgs.python3.withPackages (python-pkgs: with python-pkgs; [
          {% for dep in packages | split(pat=",") %}  {{ dep | trim }}
          {% endfor %}];
        ]);
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            python
            luajitPackages.magick # for image
          ];
        };
      };
	  });
}
