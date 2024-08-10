{
  description = "PornOS - A custom kernel. What did you though?";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem = { pkgs, system, ... }: {
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
              git
              qemu
              libisoburn
              just
              gnumake
            ] ++ [ rust-toolchain ];
          };
      };
    };
}
