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
  run    Run ram machine code from file
  check  Validates ram code syntax of a given file
  init   Generate a shell completion file
  debug  Run ram machine code and see the tape, input, output for each instruction
  help   Print this message or the help of the given subcommand(s)

Options:
  -q, --quiet    Don't pass code output to STDOUT
  -h, --help     Print help
  -V, --version  Print version
```

### Generating TAB completion

To generate TAB completion file for a given shell run

```
ram init <shell>
```

### Running code from file

```
ram run file.ram 1 2 3 4
```

### Debugging code

```
ram debug examples/three_sum.ram 1 2 3
```

Here's the debug output:

```
╭───╮
│ 0 │
├───┤
│ ? │
╰───╯
Input: 1 2 3
Output:
Next instruction: READ 1
╭───┬───╮
│ 0 │ 1 │
├───┼───┤
│ ? │ 1 │
╰───┴───╯
Input: 1 2 3
Output:
Next instruction: READ 0
╭───┬───╮
│ 0 │ 1 │
├───┼───┤
│ 2 │ 1 │
╰───┴───╯
Input: 1 2 3
Output:
Next instruction: READ 2
╭───┬───┬───╮
│ 0 │ 1 │ 2 │
├───┼───┼───┤
│ 2 │ 1 │ 3 │
╰───┴───┴───╯
Input: 1 2 3
Output:
Next instruction: ADD 1
╭───┬───┬───╮
│ 0 │ 1 │ 2 │
├───┼───┼───┤
│ 3 │ 1 │ 3 │
╰───┴───┴───╯
Input: 1 2 3
Output:
Next instruction: ADD 2
╭───┬───┬───╮
│ 0 │ 1 │ 2 │
├───┼───┼───┤
│ 6 │ 1 │ 3 │
╰───┴───┴───╯
Input: 1 2 3
Output:
Next instruction: WRITE 0
╭───┬───┬───╮
│ 0 │ 1 │ 2 │
├───┼───┼───┤
│ 6 │ 1 │ 3 │
╰───┴───┴───╯
Input: 1 2 3
Output: 6
Next instruction: HALT
```

## Roadmap for v1

- [x] Automatic changelog
- [x] Cargo crate with automatic publish to crates.io
- [x] Use clap to parse args
- [ ] Repl
- [ ] Debug mode
