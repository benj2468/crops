{
  inputs = {

    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs =
    { self
    , flake-utils
    , naersk
    , fenix
    , nixpkgs
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        toolchain =
          (fenix.packages.${system}.toolchainOf {
            sha256 = "sha256-1v11D19X2KU+ARrP8CYDip35C9E+hmJRYffZXAntY9g=";
          }).toolchain;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

        crops = naersk'.buildPackage {
          src = ./crops;
          copyLibs = true;
          nativeBuildInputs = with pkgs; [
            rust-cbindgen
            cargo-expand
          ];
          postInstall = ''
            mkdir $out/include
            cbindgen --config cbindgen.toml --crate crops --output $out/include/crops.h
          '';
        };

        crops-py = pkgs.python3Packages.callPackage ./crops/python { inherit crops; };

        shell = pkgs.mkShell {
          inputsFrom = [ crops ];
          # nativeBuildInputs = [
          #   (pkgs.python3.withPackages (p: with p; [ crops-py ]))
          # ];
        };
      in
      rec {
        # For `nix build` & `nix run`:
        packages = {
          default = crops;
          inherit crops crops-py;
        };

        # For `nix develop`:
        devShells = {
          default = shell;
        };

        formatter = pkgs.alejandra;
      }
    );
}
