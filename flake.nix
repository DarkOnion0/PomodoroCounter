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
        devShells.default = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            {
              env = {
                CARGO_INSTALL_ROOT = "${toString ./.}/.cargo";
                RUST_BACKTRACE = 1;
              };

              languages.javascript.enable = true;

              packages = with pkgs; [
                # MISC
                git

                # NIX
                alejandra

                # JS
                yarn

                # RUST
                (fenix.packages.${system}.fromToolchainFile {
                  file = ./rust-toolchain.toml;
                  sha256 = "sha256-gdYqng0y9iHYzYPAdkC/ka3DRny3La/S5G8ASj0Ayyc=";
                })

                # WASM
                wasm-pack
                openssl
                pkg-config
                binaryen
              ];

              scripts = {
                run-dev.exec = "cargo run -- ";
                run-prod.exec = "nix run .# -- ";
              };

              pre-commit.hooks = {
                # Rust
                clippy.enable = false;
                rustfmt.enable = false;
                cargo-check.enable = false;

                # Nix
                alejandra.enable = true;

                # Markdown...
                prettier.enable = true;
              };
            }
          ];
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
