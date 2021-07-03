# pkgls

[![Clippy](https://github.com/brettinternet/pkgls/actions/workflows/clippy.yml/badge.svg)](https://github.com/brettinternet/pkgls/actions/workflows/clippy.yml)
[![Test](https://github.com/brettinternet/pkgls/actions/workflows/test.yml/badge.svg)](https://github.com/brettinternet/pkgls/actions/workflows/test.yml)
[![Audit](https://github.com/brettinternet/pkgls/actions/workflows/audit.yml/badge.svg)](https://github.com/brettinternet/pkgls/actions/workflows/audit.yml)

Manage a list of installed packages for your distribution.

Supported package managers include:

- pacman

## Usage

```
USAGE:
    pkgls [OPTIONS]
    pkgls <SUBCOMMAND>

OPTIONS:
    -h, --help                 Prints help information
    -l, --log                  Increment a log level
    -p, --program <program>    Explicitly set which package manager to use [possible values: pacman]
    -q, --quiet                Silence stdout and stderr
    -V, --version              Prints version information

SUBCOMMANDS:
    install    Install packages from input or a file
    list       List installed packages or save to file
```

## Develop

### Requirements

- rust
- cargo
- make (optional)

### Run

```sh
make setup
```

```sh
make
```

### Test

```sh
make test
```
