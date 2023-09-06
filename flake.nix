{
  description = "PornOS - A custom kernel. What did you though?";

  inputs = { };

  outputs = { ... }:
    let
      forAllSystems = function:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
        ]
          (system: function nixpkgs.legacyPackages.${system});
    in
    {
      devShells = forAllSystems (pkgs: import ./shell.nix { inherit pkgs; });
    };
}
