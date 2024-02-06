# RAM Machine Interpreter

This repo contains a code for a RAM Machine code parser and interpreter as defined by Łukasz Szkup in his [Master Thesis](https://www.szkup.com/download/MaszynaRAM.pdf)

## Installation

From crates.io

```
cargo install ram-machine
```

From GitHub

```
cargo install --git https://github.com/kamack38/ram-machine
```

## Usage

```
RAM machine code interpreter

Usage: ram [OPTIONS] <COMMAND>

Commands:
  run   Run ram machine code from file
  help  Print this message or the help of the given subcommand(s)

Options:
  -i, --input-file <FILE>   Specifies the path to the input file from which data will be read (input passed from the command line takes precedence)
  -o, --output-file <FILE>  Specifies the path to the output file where the results will be written
  -q, --quiet               Don't pass code output to STDOUT
  -h, --help                Print help
  -V, --version             Print version
```

### Running code from file

```
ram run file.ram 1 2 3 4
```

## Roadmap for v1

- [x] Automatic changelog
- [x] Cargo crate with automatic publish to crates.io
- [x] Use clap to parse args
- [ ] Repl
- [ ] Debug mode
