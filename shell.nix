let
  pkgs = import <nixpkgs> {};
  crate2nix-src = pkgs.fetchFromGitHub {
    owner = "kolloch";
    repo = "crate2nix";
    rev = "0.8.0";
    sha256 = "17mmf5sqn0fmpqrf52icq92nf1sy5yacwx9vafk43piaq433ba56";
  };
  crate2nix = pkgs.callPackage (import crate2nix-src) { };
  #rustPackages = pkgs.callPackage ./rust-nightly.nix { };
in
pkgs.mkShell {
  buildInputs = (with pkgs; [
    cargo
    cmake
    crate2nix
    go
    pkgconfig
    rustc
  ]);
  /*
  ++ (with rustPackages; [
    (cargo {
      date = "2020-12-20";
      hash= "0rnbf2hb94yrs7xl567i35641z52by3jlijxwjn8m1slvnqvzshc";
    })
    (rustc {
      date = "2020-12-20";
      hash= "1asahv0lv78r2v0117hc56a62hkssnnsl6qyzyh43wrwv70jv6i7";
    })
    (rust-std {
      date = "2020-12-20";
      hash= "0x4qpwgqibljdsplrqap8r7n8kfc6lnys46c2czqva87w4fhzpwp";
    })
  ]);
  */
}
