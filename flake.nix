{
  description = "Tribeca development environment.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    saber-overlay.url = "github:saber-hq/saber-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, saber-overlay, flake-utils }:
    flake-utils.lib.eachSystem [
      "aarch64-darwin"
      "x86_64-linux"
      "x86_64-darwin"
    ]
      (system:
        let
          pkgs = import nixpkgs { inherit system; }
            // saber-overlay.packages.${system};
          ci = import ./ci.nix { inherit pkgs; };
        in
        {
          packages = {
            inherit ci;
            inherit (pkgs) anchor yarn cargo-workspaces solana-basic;
          };
          devShell = pkgs.mkShell {
            buildInputs = with pkgs; [ ci rustup cargo-deps gh ];
          };
        });
}
