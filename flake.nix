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
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
    devenv,
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
          sha256 = "sha256-gdYqng0y9iHYzYPAdkC/ka3DRny3La/S5G8ASj0Ayyc=";
        };

        craneLib =
          crane.lib.${system}.overrideToolchain fenixToolchain;

        workspace = let
          mkMember = pname: {
            inherit pname;
            cargoArtifacts = craneLib.buildDepsOnly {
              src = ./.;
              cargoToml = ./${pname}/Cargo.toml;
              cargoLock = ./Cargo.lock;
            };
            src = ./${pname};
          };
        in [
          (mkMember "cli")
          (mkMember "web")
        ];
      in {
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
              (import ./rust.nix {inherit pkgs fenixToolchain;})
              (import ./frontend.nix)
            ];
          };
          rust = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [
              defaultConfig
              (import ./rust.nix {inherit pkgs fenixToolchain;})
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

        packages = builtins.listToAttrs (
          map (
            {
              pname,
              cargoArtifacts,
              src,
            }:
              lib.nameValuePair pname (craneLib.buildPackage {
                inherit pname cargoArtifacts;
                src = ./.;
                cargoExtraArgs = "-p ${pname} -p pomolib";
              })
          )
          workspace
        );

        apps = builtins.listToAttrs (
          map (
            {pname, ...}:
              lib.nameValuePair pname {
                type = "app";
                program = "${self.packages.${system}.${pname}}/bin/${pname}";
              }
          )
          workspace
        );
      };
    };
}
