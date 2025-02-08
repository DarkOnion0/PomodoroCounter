{
  description = "A small set of rust lib/bin to provide basic pomodoro planning facilities";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
    fenix,
    crane,
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
        lib,
        ...
      }: let
        fenixToolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-vMlz0zHduoXtrlu0Kj1jEp71tYFXyymACW8L4jzrzNA=";
        };

        craneLib =
          crane.lib.${system}.overrideToolchain fenixToolchain;

        workspace = let
          mkMember = {
            name,
            profile ? builtins.null,
          }: let
            CARGO_PROFILE =
              if builtins.isNull profile
              then "release"
              else "${profile}";
            cargoArtifacts = craneLib.buildDepsOnly {
              inherit CARGO_PROFILE;
              pname =
                if builtins.isNull profile
                then "pomolib-release"
                else "pomolib-${profile}";
              src = ./.;
            };
          in {
            inherit name cargoArtifacts CARGO_PROFILE;
            pname =
              if builtins.isNull profile
              then "${name}"
              else "${name}-${profile}";
          };
        in [
          (mkMember {name = "cli";})
          (mkMember {
            name = "cli";
            profile = "dev";
          })

          (mkMember {name = "web";})
          (mkMember {
            name = "web";
            profile = "dev";
          })
        ];
      in {
        devShells = rec {
          default = pkgs.mkShell {
            shellHook = rust.shellHook;
            nativeBuildInputs = rust.nativeBuildInputs ++ base.nativeBuildInputs ++ frontend.nativeBuildInputs;
          };

          base = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              nil
              alejandra
              git
            ];
          };
          rust = pkgs.mkShell (import ./rust.nix {inherit pkgs fenixToolchain;});
          frontend = pkgs.mkShell (import ./frontend.nix {inherit pkgs;});
        };

        packages = builtins.listToAttrs (
          map (
            {
              pname,
              name,
              cargoArtifacts,
              CARGO_PROFILE,
              ...
            }:
              lib.nameValuePair pname (craneLib.buildPackage {
                inherit pname cargoArtifacts CARGO_PROFILE;
                src = ./.;
                version = (builtins.fromTOML (builtins.readFile ./${name}/Cargo.toml)).package.version;
                cargoExtraArgs = "-p ${name} -p pomolib";
              })
          )
          workspace
        );

        apps = builtins.listToAttrs (
          map (
            {
              pname,
              name,
              ...
            }:
              lib.nameValuePair pname {
                type = "app";
                program = "${self.packages.${system}.${pname}}/bin/${name}";
              }
          )
          workspace
        );
      };
    };
}
