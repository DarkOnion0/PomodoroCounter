{
  pkgs,
  fenix,
  ...
}: {
  env = {
    CARGO_INSTALL_ROOT = "${toString ./.}/.cargo";
    RUST_BACKTRACE = 1;
  };

  packages = with pkgs; [
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
  };
}
