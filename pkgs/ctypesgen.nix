{
  buildPythonPackage,
  fetchPypi,
  setuptools,
  toml,
  setuptools-scm,
}:
buildPythonPackage rec {
  pname = "ctypesgen";
  version = "1.1.1";

  format = "pyproject";

  propagatedBuildInputs = [
    setuptools
    toml
    setuptools-scm
  ];

  src = fetchPypi {
    inherit pname version;

    sha256 = "sha256-3qotZKldkBlqLoponPm5Ur5vM2b4HoNSRTVL+duskvY=";
  };
}
