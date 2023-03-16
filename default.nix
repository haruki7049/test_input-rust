with import <nixpkgs> {};
mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    clang
  ];
}
