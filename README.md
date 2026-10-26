# maturin nix template

This template is suitable for starting a project with a mixed python/rust implementation.

Features included in this template:
 - Python type-hints in user + development context,
 - PyPI publishing for many targets using Github CI,
 - `uv` managed python environment,
 - `nix flake` development environment, i.e., compatible with `$ nix develop`,
 - The permissive [MIT License](https://opensource.org/license/mit),

If you use this template and find that it is missing something that might help 
others, please leave an issue or pull request.

## Development

Development is most easily done by creating a `uv`-managed virtual environment:
```bash
uv sync
source .venv/bin/activate
```
and then locally building the package to the sourced virtual environment:
```bash
maturin develop --release --generate-stubs
```

Note that this generates the stubs required for type hints in python, automatically generated from the rust function signatures.

Typically, I will experiment with the package using scripts in the `./test` directory, but canonically this directory should contain actual tests to be run (e.g.) with `pytest`:

```bash
pytest --cov=python tests/ --cov-report html
```

## Releasing
To release the project on PyPI, first add your `PYPI_API_TOKEN` to Settings/Secrets and Variables/Repository Secrets on Github.

Then, each time you want to publish a release, increment the `version` in `./Cargo.toml` appropriately, commit the tracked files, and then run:
```bash
git tag <VERSION>
git push --follow-tags
```
This will initiate a CI test on github, and if successful, will upload the built wheels/new version to PyPI.