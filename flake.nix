{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    fenix,
    nixpkgs,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
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
          })
          .toolchain;

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
          inputsFrom = [crops];

          nativeBuildInputs = with pkgs; [ctypesgen rust-cbindgen simple.python];
        };

        mkExample = {name}: let
          lib = naersk'.buildPackage {
            inherit name;
            src = ./examples + "/${name}";
            copyLibs = true;
            nativeBuildInputs = with pkgs; [cargo-expand rust-cbindgen];
            preBuild = ''
              ln -s ${./crops} crops
            '';
            postInstall = ''
              cbindgen --config $src/cbindgen.toml --crate ${name} --output $out/include/${name}.h
            '';
          };

          python = let
            _pyproject = pkgs.writeTextFile {
              name = "${name}-pyproject.toml";
              text = ''
                [project]
                name = "${name}"
                version = "0.1.0"
              '';
            };

            _src = pkgs.runCommand "${name}-py-src" {} ''
              mkdir $out

              ln -s ${_pyproject} $out/pyproject.toml
            '';
          in
            pkgs.python3Packages.buildPythonPackage {
              inherit name;
              src = _src;

              format = "pyproject";

              nativeBuildInputs = with pkgs.python3Packages; [
                pkgs.ctypesgen
              ];

              propagatedBuildInputs = [lib];

              preBuild = ''
                mkdir -p src/${name}
                ctypesgen -L${lib}/lib -l${name} ${lib}/include/*.h -o src/${name}/__init__.py
              '';
            };
        in {inherit lib python;};

        simple = mkExample {
          name = "simple";
        };
      in rec {
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
