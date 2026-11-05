let
  pkgs = import <nixpkgs> {
    config = {
      allowUnfree = true;
    };
    overlays = [ ];
  };
in


pkgs.mkShellNoCC {
  packages = with pkgs; [
    nixd
    cargo
    rustc
    rustfmt
    rust-analyzer
    clippy
  ];
}
