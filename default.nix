{
  pkgs ? import <nixpkgs> { },
}:
let
  mainPkg = pkgs.rustPlatform.buildRustPackage {
    pname = "mask-hx";
    version = "0.4.0";
    src = pkgs.lib.cleanSource ./.;
    cargoLock.lockFile = ./Cargo.lock;
  };
in
pkgs.buildFHSEnv rec {
  name = "mask-hx-fhs";
  executableName = "mask-hx";
  targetPkgs = pkgs: [
    mainPkg
    pkgs.neko
  ];
  runScript = "${mainPkg}/bin/${executableName}";
}
