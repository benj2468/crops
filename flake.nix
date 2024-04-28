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
          overlays = [
            (final: prev: {
              ctypesgen = prev.python3Packages.callPackage ./pkgs/ctypesgen.nix {};
            })
          ];
        };

        toolchain =
          (fenix.packages.${system}.toolchainOf {
            sha256 = "sha256-IUZ+84Dg0ubNlH8jwG0hRa4F9Ye4uRYBNzC90+eqalc=";
          }).toolchain;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

        crops = naersk'.buildPackage {
          src = ./crops;
          copyLibs = true;
          nativeBuildInputs = with pkgs; [
            cargo-expand
          ];
        };

        shell = pkgs.mkShell {
          inputsFrom = [ crops ];

          nativeBuildInputs = with pkgs; [ ctypesgen rust-cbindgen ]; 
        };

        mkExample = { name }: naersk'.buildPackage {
          inherit name;
          src = ./examples + "/${name}";
          copyLibs = true;
          nativeBuildInputs = with pkgs; [ cargo-expand rust-cbindgen ];
          preBuild = ''
            ln -s ${./crops} crops
          '';
          postInstall = ''
            mkdir $out/include
            cbindgen --config $src/cbindgen.toml --crate ${name} --output $out/include/${name}.h
          '';
        };

        simple = mkExample {
          name = "simple";
        };

      in
      rec {
        # For `nix build` & `nix run`:
        packages = {
          default = crops;
          inherit simple;
        };

        # For `nix develop`:
        devShells = {
          default = shell;
        };

        formatter = pkgs.alejandra;
      }
    );
}
