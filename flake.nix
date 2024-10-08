{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-parts.url = "github:hercules-ci/flake-parts";
    pwndbg.url = "github:pwndbg/pwndbg";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; }
      {
        systems = [
          "x86_64-linux"
          "aarch64-linux"
        ];

        perSystem = { self', lib, system, pkgs, config, ... }: {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;

            overlays = with inputs; [
              rust-overlay.overlays.default
            ];
          };

          devShells.default =
            let
              rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
            in
            pkgs.mkShell {
              packages = with pkgs; [
                gnumake
                qemu_full
                libisoburn
                just
                inputs.pwndbg.packages.${system}.default
              ] ++ [ rust-toolchain ];
            };
        };
      };
}
