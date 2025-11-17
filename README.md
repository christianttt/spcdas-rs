# spcdas-rs

A disassembler for the Sony SPC700 processor, written in Rust.

This project is a feature-compatible port of the original `spcdas` command-line utility. It aims to provide the same functionality and command-line interface.

## Acknowledgements

All credits for the original implementation goes to **byuu** and **gocha**.

The original source code can be found at: **[gocha/spcdas](https://github.com/gocha/spcdas)**

## Planned Features

* Fuller rewrite (code is a mess right now)



## Usage

The command-line interface is designed to be compatible with the original `spcdas`.

```
spcdas-rs <input_file> <output_file> [OPTIONS]
```

### Options

| Option                | Description                                                          | Default   |
| --------------------- | -------------------------------------------------------------------- | --------- |
| `--load <ADDR>`       | Memory address to load the input file at [hex].                      | `0000`    |
| `--pc <ADDR>`         | Address to start disassembling from [hex].                           | `0000`    |
| `--stop <ADDR\|eof>`  | Address to stop disassembly at [hex or "eof"].                       | `eof`     |
| `--no-addr`           | Disable displaying the memory address for each instruction.          | (flag)    |
| `--no-hex`            | Disable displaying the hex bytes for each instruction.               | (flag)    |
| `--no-rel-resolve`    | Display raw relative branch offsets instead of resolved addresses.   | (flag)    |

