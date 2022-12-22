{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; # We want to use packages from the binary cache
    flake-utils.url = "github:numtide/flake-utils";
    gitignore = { url = "github:hercules-ci/gitignore.nix"; flake = false; };
  };

  outputs = inputs@{ self, nixpkgs, flake-utils, ... }:
  flake-utils.lib.eachSystem [ flake-utils.lib.system.x86_64-linux flake-utils.lib.system.aarch64-linux flake-utils.lib.system.i686-linux ] (system: let
    inherit (builtins) substring;

    # to work with older version of flakes
    lastModifiedDate = self.lastModifiedDate or self.lastModified or "19700101";

    # Generate a user-friendly version number.
    version = builtins.substring 0 8 lastModifiedDate;

    pkgs = nixpkgs.legacyPackages.${system};
    gitignoreSrc = pkgs.callPackage inputs.gitignore { };
  in rec {
    packages.pomodoro_counter = pkgs.callPackage ./default.nix { inherit gitignoreSrc; };

    legacyPackages = packages;

    defaultPackage = packages.pomodoro_counter;

    devShell = pkgs.mkShell {
      CARGO_INSTALL_ROOT = "${toString ./.}/.cargo";

      buildInputs = with pkgs; [ git rustup ];
    };

    apps = rec {
      default = pomodoro_counter;
      pomodoro_counter = {
        type = "app";
        program = "${self.packages.${system}.default}/bin/pomodoro_counter";
      };
    };
  });
}
