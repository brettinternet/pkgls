# pkgls

List installed packages.

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
    -p, --program <program>    Explicitly set which package manager to use
    -q, --quiet                Silence stdout and stderr
    -V, --version              Prints version information

SUBCOMMANDS:
    list    List installed packages or save to file
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
