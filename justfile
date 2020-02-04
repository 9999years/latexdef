build:
    nix-build -E "{ pkgs ? import <nixpkgs> { }, }: pkgs.callPackage ./default.nix { }"