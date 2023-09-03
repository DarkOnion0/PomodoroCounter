{
  pkgs,
  fenixToolchain,
  ...
}: {
  env = {
    CARGO_INSTALL_ROOT = "${toString ./.}/.cargo";
    RUST_BACKTRACE = 1;
  };

  packages = with pkgs; [
    # RUST
    fenixToolchain

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
}
