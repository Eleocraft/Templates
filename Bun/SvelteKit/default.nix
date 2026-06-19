{
  bun2nix,
  runCommand,
  stdenv,
  lib,
  ...
}:
let
  # Generated at build time from bun.lock so we don't keep a second file in
  # sync. bun2nix is a pure transformation here — bun.lock already contains
  # integrity hashes for every dependency, so no network access is needed.
  # This uses import-from-derivation: evaluation pauses until bun.nix is built.
  bunNix = runCommand "bun.nix" { nativeBuildInputs = [ bun2nix ]; } ''
    bun2nix -l ${./bun.lock} -o $out
  '';
in
bun2nix.writeBunApplication {
  packageJson = ./package.json;

  # Filter out untracked dev artifacts. A `path:` flake input copies the whole
  # working directory (unlike a git fetch, which only sees tracked files), so a
  # stale local node_modules / .next would otherwise pollute the build and its
  # `#!/usr/bin/env node` bin shebangs break in the sandbox. bun2nix reinstalls
  # node_modules from bun.nix, so dropping these is safe.
  src = lib.cleanSourceWith {
    src = ./.;
    filter =
      path: type:
      !(builtins.elem (baseNameOf (toString path)) [
        "node_modules"
        ".direnv"
        ".next"
        "result"
      ]);
  };

  bunInstallFlags = [
    "--cpu=*"
  ];

  buildPhase = ''
    bun run build
  '';

  # nextjs needs to bind to a port during the build process
  __darwinAllowLocalNetworking = true;

  startScript = ''
    bun run start
  '';

  bunDeps = bun2nix.fetchBunDeps {
    inherit bunNix;

    # GitHub Packages auth for @tilestion/*. bun2nix reads //<host>/:_authToken
    # from this .npmrc and injects an `Authorization: Bearer` header into the
    # fetchurl for matching registries. Sourced from $HOME so the token is not
    # committed to git — this requires building with `--impure`.
    npmrcPath = "${builtins.getEnv "HOME"}/.npmrc";
  };
}
