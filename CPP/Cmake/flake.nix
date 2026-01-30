{
  description = "Linux devshell for CMake";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: 
  let
    pkgs = import nixpkgs { system = "x86_64-linux"; };
  in
  {
    devShells.x86_64-linux.default = pkgs.mkShell.override { inherit (pkgs.llvmPackages_latest) stdenv; } {
      # Binary dependencies
      buildInputs = with pkgs; [
        drogon # example
      ];
      # Build dependencies
      nativeBuildInputs = with pkgs; [
        cmake
        ninja
      ];

      shellHook = ''
        ./generate_compile_flags.sh
      '';
    };
  };
}
