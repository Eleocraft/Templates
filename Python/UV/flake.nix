{
  description = "Python development environment with uv for fast package management";

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
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            python3
            python313Packages.python-lsp-server
            uv  # Fast Python package installer and resolver
            
            # Build dependencies that might be needed
            gcc
            zlib
            openssl
            libffi
            pkg-config
          ];
          
          shellHook = ''
            # Create virtual environment with uv if it doesn't exist
            if [ ! -d ".venv" ]; then
              echo "Creating virtual environment with uv..."
              uv venv
            fi
            
            # Activate virtual environment
            source .venv/bin/activate
            
            # Install requirements if requirements.txt exists
            if [ -f "requirements.txt" ]; then
              echo "Installing requirements from requirements.txt with uv..."
              uv pip install -r requirements.txt
            fi
            
            # Install pyproject.toml dependencies if they exist
            if [ -f "pyproject.toml" ]; then
              echo "Installing dependencies from pyproject.toml with uv..."
              uv pip install -e .
            fi
            
            echo "Python environment with uv activated"
            echo "Python: $(python --version)"
            echo "UV: $(uv --version)"
            echo "Virtual environment: $VIRTUAL_ENV"
          '';
        };
      };
    });
}
