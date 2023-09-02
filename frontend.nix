{pkgs, ...}: {
  languages.javascript.enable = true;

  packages = with pkgs; [
    # JS
    yarn
  ];

  pre-commit.hooks = {
    # Nix
    alejandra.enable = true;

    # Markdown...
    prettier.enable = true;
  };
}
