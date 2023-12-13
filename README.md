# dev-tools-rust

This is an experimental project as trying to learn Rust.

It is planned to have set of commands to improve development process.

Command name is `dt`


## `dt pkg`

This is just short for `venv/bin/pip install -r requirements.txt` with some
extra abilities.

Commands:
- `dt pkg -c` compiles all `requirements*.in` files using
  [pip-tools](https://pip-tools.readthedocs.io/en/latest/).
- `dt pkg -i` installs all `requirements*.txt` files.

`-c` and `-i` commands can be combined to compile and install all packages.

Features:
- [x] Only uses virtualenv named `venv`. Stops if there is none.
- [x] Searches all `requirements*.txt` files and installs all of them.
- [x] Complies all `requirements*.txt` files into same file name with `txt` extension.

Planned:
- [ ] Ability to specify requirement file for compiling / installing.
- [ ] Ability to specify virtualenv folder name.
- [ ] Make installing a default ability.
  - [ ] Remove `-i` flag.
  - [ ] Add `--no-install` flag.


## Configuration

Currently there is no configuration support.

Planned:
- [ ] Add ability to read configuration from `pyproject.toml`
