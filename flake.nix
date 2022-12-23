{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; # We want to use packages from the binary cache

    flake-utils.url = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    devenv.url = "github:cachix/devenv";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-utils,
    fenix,
    crane,
    devenv,
    ...
  }:
    flake-utils.lib.eachSystem [flake-utils.lib.system.x86_64-linux flake-utils.lib.system.aarch64-linux flake-utils.lib.system.i686-linux] (system: let
      inherit (builtins) substring;

      # to work with older version of flakes
      pkgs = nixpkgs.legacyPackages.${system};

      rust-toolchain = fenix.packages.${system}.stable;

      craneLib = crane.lib.${system}.overrideToolchain rust-toolchain.completeToolchain;
    in rec {
      packages = rec {
        default = pomodoro_counter;

        pomodoro_counter = craneLib.buildPackage {
          src = ./.;
        };
      };

      legacyPackages = packages;

      defaultPackage = packages.pomodoro_counter;

      devShell = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [
          {
            env.CARGO_INSTALL_ROOT = "${toString ./.}/.cargo";

            packages = with pkgs; [
              git
              alejandra
              rust-toolchain.completeToolchain
              rust-analyzer
            ];

            scripts = {
              run-dev.exec = "cargo run -- ";
              run-prod.exec = "nix run .# -- ";
            };

            pre-commit.hooks = {
              # Rust
              clippy.enable = true;
              rustfmt.enable = true;
              cargo-check.enable = true;

              # Nix
              alejandra.enable = true;

              # Markdown...
              prettier.enable = true;
            };
          }
        ];
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
