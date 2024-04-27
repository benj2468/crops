{
  buildPythonPackage,
  gcc,
  setuptools,
  ctypesgen,
  crops,
  ...
}:
buildPythonPackage rec {
  name = "crops";
  src = ./.;
  format = "pyproject";

  nativeBuildInputs = [gcc setuptools ctypesgen];

  propagatedBuildInputs = [crops];

  preBuild = ''
    PROJECT_DIR=src/${name}
    mkdir -p $PROJECT_DIR

    ctypesgen -L${crops}/lib -l${name} ${crops}/include/*.h -o $PROJECT_DIR/__init__.py
  '';
}
