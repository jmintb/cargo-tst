{ pkgs ? import <nixpkgs> {} }:
(pkgs.buildFHSUserEnv {
  name = "pythondev";
  targetPkgs = pkgs: (with pkgs; [
    gcc
    binutils-unwrapped
    rustup
  ]);
  runScript = "bash";
}).env

