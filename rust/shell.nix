{ pkgs ? import <nixpkgs> { } }:

let
  np = import (builtins.fetchTarball {
    url =
      "https://github.com/NixOS/nixpkgs/archive/9256f7c71a195ebe7a218043d9f93390d49e6884.tar.gz";
  }) { };

  myPkg = np.nodejs_20;

in pkgs.mkShell {
  buildInputs = with pkgs; [
    xorg.libxcb
    pkgconfig
    xorg.libX11
    # nodejs_20
    myPkg
  ];
}
