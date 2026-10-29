{
  description = "Advent of Code polyglot playground";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};

        rust = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust
            rustc
            cargo
            clippy
            rustfmt
          ];
        };
      in {
        devShells = {
          inherit rust;

          default = pkgs.mkShell {
            # pull in every language shell, so `just` works from the repo root
            inputsFrom = [rust];

            buildInputs = with pkgs; [
              just
              curl
            ];
          };
        };
      }
    );
}
