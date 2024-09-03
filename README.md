# RustyFileNinja

This is a simple file manager written in [Rust](https://www.rust-lang.org/) that allows you to:

* Create a file (empty or with optional text)
* Copy a file
* Combine two files into a third
* Delete a file

## Installation

### Pre-requisites:

* `wget` -or- `curl`

### Install App 

Depending on if you prefer `wget` or `curl`, run one of the commands below to download the app:

```
sh -c "$(curl -fsSL https://raw.githubusercontent.com/smyleeface/RustyFileNinja/main/install.sh)"
```
```
sh -c "$(wget https://raw.githubusercontent.com/smyleeface/RustyFileNinja/main/install.sh -O -)"
```

### Uninstall App 

Depending on if you prefer `wget` or `curl`, run one of the commands below to download the app:

```
sh -c "$(curl -fsSL https://raw.githubusercontent.com/smyleeface/RustyFileNinja/main/uninstall.sh)"
```
```
sh -c "$(wget https://raw.githubusercontent.com/smyleeface/RustyFileNinja/main/uninstall.sh -O -)"
```

Follow the on-screen instructions to finish the setup. 

## Usage

`file-ninja help`

```bash
Usage: file-ninja <COMMAND>

Commands:
  create       Creates a new file with or without content
  copy         Copies a file to another location when given both the source and destination
  combine      Creates a new file with the contents of two provided files
  delete       Deletes a file given the filename
  create-docs  Creates man docs for the cli tool
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Development

* Clone repo
* From the repo root run `cargo build` to build
* From the repo root run `cargo run <COMMAND>` -or- `./target/debug/file-ninja <COMMAND>` to run application

### Adding new commands

Add new commands to the `src/cli.rs` file and import the command it should run.

### Running Tests

From the repo root run:

```
cargo test
```

### Updating man docs

From the repo root run:

```
cargo run create-docs
```
-OR-
```
file-ninja create-docs
```
