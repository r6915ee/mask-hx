{
  pkgs ? import <nixpkgs> { },
}:
(pkgs.buildFHSEnv {
  name = "mask-hx";

  targetPkgs =
    pkgs: with pkgs; [
      rustc
      cargo
      rustfmt
      rust-analyzer
      clippy
      neko
      gcc
    ];

  RUST_BACKTRACE = 1;
}).env
