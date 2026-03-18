{
  description = "MDot toolchain";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        # built packages and windows cross-compilation
        packages.default = pkgs.callPackage ./default.nix { };

        # devshell for ease of use
        devShells.default = pkgs.mkShell {
          name = "mdotshell";
          inputsFrom = [ self.packages."${system}".default ];
        };
      }
    );
}
