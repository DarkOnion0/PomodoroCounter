{ pkgs ? import <nixpkgs> { }
, stdenv ? pkgs.stdenv
, lib ? pkgs.lib
# A set providing `buildRustPackage :: attrsets -> derivation`
, rustPlatform ? pkgs.rustPlatform
, fetchFromGitHub ? pkgs.fetchFromGitHub
, gitignoreSrc ? null
}:

let
  gitignoreSource =
    if gitignoreSrc != null
    then gitignoreSrc.gitignoreSource
    else (import (fetchFromGitHub {
      owner = "hercules-ci";
      repo = "gitignore";
      rev = "c4662e662462e7bf3c2a968483478a665d00e717";
      sha256 = "0jx2x49p438ap6psy8513mc1nnpinmhm8ps0a4ngfms9jmvwrlbi";
    }) { inherit lib; }).gitignoreSource;
in
rustPlatform.buildRustPackage rec {
  pname = "pomodoro_counter";
  version = "0.1.0";

  src = gitignoreSource ./.;

  cargoSha256 = "sha256-Eha/5TRcmHuxmxtcTHu/xKAZuWERRmzj3U1dnfTlbZU=";

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = with lib; {
    description = "An app to convert pomodoros to real hours";
    license = licenses.agpl3;
  };
}
