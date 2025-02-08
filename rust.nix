{
  pkgs,
  fenixToolchain,
  ...
}: {
  shellHook = ''
    export RUST_BACKTRACE=1
    export CARGO_INSTALL_ROOT="${toString ./.}/.cargo"
  '';

  nativeBuildInputs = with pkgs; [
    # RUST
    fenixToolchain

    # WASM
    wasm-pack
    openssl
    pkg-config
    binaryen
  ];
  #scripts = {
  #  run-dev.exec = "cargo run -- ";
  #  run-prod.exec = "nix run .# -- ";
  #};
}
