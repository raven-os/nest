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
nest [OPTION]... SUBCOMMAND [SUBCOMMAND OPTIONS]...

Raven-OS's package manager.

OPTIONS
    -h, --help       Prints help information
    -v               Set the level of verbosity
    -V, --version    Prints version information

SUBCOMMANDS
    help         Prints this message or the help of the given subcommand(s)
    install      Download and install the given packages [alias: add]
    pull         Pull repositories and update the local cache
    uninstall    Uninstall the given packages [alias: remove]
    upgrade      Upgrade all installed packages [alias: update]
```

```
finest [OPTION]... SUBCOMMAND [SUBCOMMAND OPTIONS]...

Raven-OS's package manager.

OPTIONS
    -h, --help       Prints help information
    -v               Set the level of verbosity
    -V, --version    Prints version information

SUBCOMMANDS
    group          Operate on groups
    help           Prints this message or the help of the given subcommand(s)
    merge          Merge the scratch dependency graph with the regular dependency graph
    pull           Pull repositories and update the local cache
    requirement    Operate on requirements
```

