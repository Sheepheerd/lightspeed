# shell.nix
{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = [

    pkgs.pylint
    pkgs.python312
    pkgs.python312Packages.virtualenv
    pkgs.gtk3
    pkgs.gtk4
    pkgs.cairo
    pkgs.gtk-layer-shell
    pkgs.gobject-introspection
    # pkgs.python312Packages.pygobject3
    pkgs.python312Packages.tkinter
    pkgs.python312Packages.pycairo
    pkgs.python312Packages.loguru
    pkgs.python312Packages.pynput
    pkgs.pkgconf

  ];

  shellHook = ''
    export PYTHONPATH=${pkgs.python3Packages.pygobject3}/lib/python3.12/site-packages:$PYTHONPATH
  '';
}

