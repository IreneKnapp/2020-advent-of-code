let
  pkgs = import <nixpkgs> {};
  crate2nix-src = pkgs.fetchFromGitHub {
    owner = "kolloch";
    repo = "crate2nix";
    rev = "0.8.0";
    sha256 = "17mmf5sqn0fmpqrf52icq92nf1sy5yacwx9vafk43piaq433ba56";
  };
  crate2nix = pkgs.callPackage (import crate2nix-src) { };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    cmake
    crate2nix
    go
    pkgconfig
    rustc
  ];
}
