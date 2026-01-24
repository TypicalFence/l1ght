{
  description = "l1ght";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "i686-linux"
      ];
      eachSystem = nixpkgs.lib.genAttrs systems;

    in
    {
      formatter = eachSystem (system: nixpkgs.legacyPackages.${system}.nixfmt);
      packages = eachSystem (system: {
        l1ght-debug = nixpkgs.legacyPackages.${system}.callPackage ./nix/package.nix { debug = true; };
        l1ght = nixpkgs.legacyPackages.${system}.callPackage ./nix/package.nix { };
        default = self.packages.${system}.l1ght;
      });
    };
}
