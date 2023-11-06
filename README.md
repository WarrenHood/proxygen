# Proxify

A DLL proxy generator written in Rust. Easily proxy any DLL.

Features:
- Dump exported DLL function names
- Generate proxy DLL Rust project
- Merge new DLL exports into an existing proxy DLL project
- Update an existing DLL project's exports (automatically unproxies intercepted functions)

## Usage

```
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

## Example usage

```bash
proxify generate path/to/some_library.dll my_some_library_proxy
```

And just like that, you have a ready to compile DLL proxy Rust project.

Then add some exports you want to replace to `intercepted_exports.rs`.

Eg. Something like:

```rust
#[no_mangle]
pub extern "C" fn some_dll_export(x: u64, y: u64) -> u64 {
    println!("Proxy some_dll_export function called...");
    5
}
```

And then update your exports by running this in the root of the project before building:
```bash
proxify update .
```