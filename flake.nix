{
  description = "Dev shell with Rust and ra-multiplex";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";

  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

      in {
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              pkg-config
              gtk3
              gtk4
              glib
              rust-bin.beta.latest.default
              rust-analyzer
              ra-multiplex
            ];
          };

        shellHook = "";

      });
}
