{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; # We want to use packages from the binary cache

    flake-parts.url = "github:hercules-ci/flake-parts";

    devenv.url = "github:cachix/devenv";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
    fenix,
    devenv,
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
              env.CARGO_INSTALL_ROOT = "${toString ./.}/.cargo";

              languages.rust.enable = true;

              packages = with pkgs; [
                git
                alejandra
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
      };
    };
}
