{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    devshell.url = "github:numtide/devshell";
    naersk = {
      url = "github:nix-community/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    devshell,
    naersk,
    fenix,
    ...
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            devshell.overlay
            fenix.overlay
            (final: prev: {
              rustWithComponents = prev.fenix.complete.withComponents [
                "cargo"
                "clippy"
                "rust-src"
                "rustc"
                "rustfmt"
              ];
            })
          ];
        };
        lib = pkgs.lib;
        rust-nightly = fenix.packages.${system};
        naersk-lib = let
          toolchain = with rust-nightly;
            combine (with minimal; [
              cargo
              rustc
            ]);
        in
          naersk.lib.${system}.override {
            cargo = toolchain;
            rustc = toolchain;
          };
        buildInputs = with pkgs; [
          openssl
          pkg-config
        ];
      in rec {
        packages.default = naersk-lib.buildPackage {
          pname = "cook";
          src = ./.;
          inherit buildInputs;
        };

        apps.default = utils.lib.mkApp {drv = packages.default;};

        devShells.default = pkgs.devshell.mkShell {
          imports = [
            (pkgs.devshell.importTOML ./devshell.toml)
          ];
        };
      }
    );
}
