{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = [

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

  languages.python = {
    enable = true;

    venv.enable = true;
    venv.requirements = ./requirements.txt;

  };

  # # https://devenv.sh/scripts/
  # scripts.hello.exec = ''
  #   echo hello from $GREET
  # '';

  # enterShell = "\n";

  # https://devenv.sh/tests/
  enterTest = ''
    # echo "Running tests"
    # git --version | grep --color=auto "${pkgs.git.version}"
  '';

}
