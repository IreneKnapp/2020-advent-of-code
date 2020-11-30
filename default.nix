# Use `nix-build`, not `nix build`, when doing development on this file. It
# does a better job of propagating error messages from the build tools upward.

let pkgs = import <nixpkgs> { };
    crateOverrides = pkgs.defaultCrateOverrides.override {
    } // { };
    parameterOverrides = { };
    cargo = pkgs.callPackage ./Cargo.nix {
      defaultCrateOverrides = crateOverrides;
    };
in
builtins.mapAttrs
    (key: value: value.build.override parameterOverrides)
    cargo.workspaceMembers
