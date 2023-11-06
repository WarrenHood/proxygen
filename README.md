# Proxify

A WIP DLL proxy generator.

Features:
- Dump exported DLL function names
- Generate proxy DLL Rust project
- Merge new DLL exports into an existing proxy DLL project
- Update an existing DLL project's exports (automatically unproxies intercepted functions)

## Usage

```bash
A DLL export dumper and proxy generator

Usage: proxify <COMMAND>

Commands:
  dump-exports  Prints out the exported functions from a given PE file
  generate      Generate a new proxy DLL project for the given DLL file
  merge         Merges the given DLL's new exports into an existing DLL proxy project
  update        Updates an exisitng DLL proxy project's exports based on the intercepted exports
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Note: Generated projects need to be built with nightly Rust.