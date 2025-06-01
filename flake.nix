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
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      ...
    }:
    let
      supportedSystems = [ "x86_64-linux" ];

      forSupportedSystems =
        generator:
        let
          generateForSystem =
            system:
            generator rec {
              inherit system;
              pkgs = nixpkgs.legacyPackages.${system};
              craneLib = crane.mkLib pkgs;
            };
        in
        nixpkgs.lib.genAttrs supportedSystems generateForSystem;
    in
    {
      packages = forSupportedSystems (
        { pkgs, craneLib, ... }:
        {
          github-app = craneLib.buildPackage {
            src = craneLib.cleanCargoSource (craneLib.path ./.);

            strictDeps = true;

            buildInputs = [
              pkgs.openssl
              pkgs.pkg-config
            ] ++ pkgs.lib.optional pkgs.stdenv.isDarwin pkgs.libiconv;

            meta = with pkgs.lib; {
              description = "GitHub Koritsu application";
              license = licenses.mit;
              platforms = platforms.linux;
              mainProgram = "koritsu-app";
            };
          };
        }
      );

      devShells = forSupportedSystems (
        {
          system,
          pkgs,
          craneLib,
          ...
        }:
        let
          fix = pkgs.writeShellScriptBin "fix" ''
            set -e

            nix fmt
            dprint fmt
            cargo fmt
            cargo clippy --fix --allow-dirty --allow-staged --all-targets
          '';

          checkFmt = pkgs.writeShellScriptBin "chkfmt" ''
            set -e

            nix fmt . -- --check
            dprint check
            cargo fmt --check
          '';

          lint = pkgs.writeShellScriptBin "lint" ''
            cargo clippy --all-targets -- --deny warnings
          '';
        in
        {
          default = craneLib.devShell {
            inputsFrom = [ self.packages.${system}.github-app ];

            # This environment variable is required by rust-analyzer
            # to find the source and expand proc macros. See:
            # https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

            packages = with pkgs; [
              fix
              checkFmt
              lint
              dprint
              cocogitto
            ];
          };
        }
      );

      formatter = forSupportedSystems ({ pkgs, ... }: pkgs.nixfmt-tree);
    };
}
