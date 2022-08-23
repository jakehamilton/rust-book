{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-22.05";
    unstable.url = "github:nixos/nixpkgs";

    flake-utils-plus = {
      url = "github:gytis-ivaskevicius/flake-utils-plus";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "unstable";
    };
  };

  outputs = inputs@{ self, nixpkgs, unstable, flake-utils-plus, fenix }:
    flake-utils-plus.lib.mkFlake {
      inherit self inputs; 

      outputsBuilder = channels:
        let
          pkgs = channels.nixpkgs;
          inherit (fenix.packages.${pkgs.system}) stable;
        in {
          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [ stable.toolchain ];
          };
        };
    };
}
