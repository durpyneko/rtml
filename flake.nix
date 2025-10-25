{
  description = "html in rust";
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs, ... }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
    };
  in {
    devShells = {
      x86_64-linux.default = pkgs.mkShell rec {
        buildInputs = with pkgs; [
          pkg-config gcc
          rustup
        ];

        shellHook = ''
          rustup default 1.89.0
          rustup component add rust-src rust-std
          rustup component add rust-docs rust-analyzer
        '';
      };
    };
  };
}