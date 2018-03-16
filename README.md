# Nest [![Build Status](https://travis-ci.org/raven-os/nest.svg?branch=master)](https://travis-ci.org/raven-os/nest)

Raven's package manager.

## Build dependencies
* rustup, with the latest nightly toolchain available

## Building Nest

Compiling Nest is pretty straightforward:

```bash
cargo build --all
```

## Running tests

If you want to be sure everything went correctly when compiling Nest, you can run the tests:

```bash
cargo test --all
```
 
## Running Nest

You shouldn't run Nest in an un-protected environnement, as Nest assumes there is no other package manager on the current system.

You can still run it with:

```bash
cd nest
cargo run -- <args>
```

See below for available arguments.

## Usage

```
nest [FLAGS] OPERATION
Raven's package manager

FLAGS
    -h, --help       Prints help information
    -v               Sets the level of verbosity
    -V, --version    Prints version information

OPERATIONS
    help         Prints this message or the help of the given subcommand(s)
    install      Installs the given packages
    list         List informations about installed packages
    pull         Pulls repositories and updates local cache
    search       Search the local database for packages
    uninstall    Uninstalls the given packages [aliases: remove]
    upgrade      Upgrades installed packages
```
