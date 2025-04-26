# This file is part of koritsu
#
# Copyright (c) 2025 Thomas Himmelstoss
#
# This software is subject to the MIT license. You should have
# received a copy of the license along with this program.

{
  description = "Build koritsu-app";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { nixpkgs, ... }:
    let
      supportedSystems = [ "x86_64-linux" ];

      forSupportedSystems = generator:
        let
          generateForSystem = system: generator {
            inherit system;
            pkgs = nixpkgs.legacyPackages.${system};
          };
        in
        nixpkgs.lib.genAttrs supportedSystems generateForSystem;
    in
    {
      devShells = forSupportedSystems ({ system, pkgs, ... }:
        let
          fix = pkgs.writeShellScriptBin "fix" ''
            set -e

            nix fmt
            dprint fmt
          '';
          checkFmt = pkgs.writeShellScriptBin "chkfmt" ''
            set -e

            nix fmt . -- --check
            dprint check
          '';
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              fix
              checkFmt
              dprint
            ];
          };
        });

      formatter = forSupportedSystems ({ pkgs, ... }: pkgs.nixpkgs-fmt);
    };
}
