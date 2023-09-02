{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    devenv.url = "github:cachix/devenv";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
    devenv,
    fenix,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      debug = true;
      systems = ["x86_64-linux" "aarch64-linux"];
      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        ...
      }: {
        devShells = let
          defaultConfig = {
            packages = with pkgs; [
              # MISC
              git

              # NIX
              alejandra
            ];
          };
        in {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [
              defaultConfig
              (import ./rust.nix {inherit pkgs fenix;})
              (import ./frontend.nix)
            ];
          };
          rust = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [
              defaultConfig
              (import ./rust.nix {inherit pkgs fenix;})
            ];
          };
          frontend = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [
              defaultConfig
              (import ./frontend.nix)
            ];
          };
        };

        packages = rec {
          default = cli;
          cli = pkgs.rustPlatform.buildRustPackage rec {
            pname = "cli";
            version = "0.1.0";
            src = ./.;
            cargoBuildFlags = "-p ${pname}";

            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
          web = pkgs.rustPlatform.buildRustPackage rec {
            pname = "web";
            version = "0.1.0";
            src = ./.;
            cargoBuildFlags = "-p ${pname}";

            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
        };

        apps = rec {
          default = cli;
          cli = {
            type = "app";
            program = "${self.packages.${system}.cli}/bin/cli";
          };
          web = {
            type = "app";
            program = "${self.packages.${system}.web}/bin/web";
          };
        };
      };
    };
}
