{pkgs ? import <nixpkgs> {}}:
{
  default = pkgs.mkShell {
    packages = with pkgs; [
      git
      qemu
      libisoburn
    ];
  };
}
