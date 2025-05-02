{
  description = "Dev shell with Rust, GTK3, GTK4, and friends";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; };
      in {
        devShells.default = pkgs.mkShell {

          buildInputs = with pkgs; [
            pkg-config
            gtk3
            gtk4
            glib
            rustc
            cargo
            rustup
          ];
        };
      });
}
