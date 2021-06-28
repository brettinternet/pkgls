# pkgls

List installed packages.

Supported package managers include:

- pacman

## Usage

```
USAGE:
    pkgls [OPTIONS] [OUTPUT]

ARGS:
    <OUTPUT>    Filename to write package names

OPTIONS:
    -f, --force                Force overwrite the output if it already exists
    -h, --help                 Prints help information
    -l, --log                  Increment a log level
    -p, --program <program>    Explicitly set which package manager to use
    -q, --quiet                Silence stdout and stderr
    -V, --version              Prints version information
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
