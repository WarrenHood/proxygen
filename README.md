# proxygen
[![Crates.io](https://img.shields.io/crates/v/proxygen)](https://crates.io/crates/proxygen)

A DLL proxy generator written in Rust. Easily proxy any DLL.

Features:
- Generate a proxy DLL Rust project
- Merge new DLL exports into an existing proxy DLL project
- Update an existing DLL project's exports (removes automatically generated proxies which have been intercepted)

## Installing

Assuming you have [installed Rust](https://rustup.rs/), just run:

```bash
cargo install proxygen
```

## Editing the generated project

You generally just need to edit `src/intercepted_exports.rs`.

Just add whatever functions you want to intercept to `src/intercepted_exports.rs` (make sure to match the name in `src/proxied_exports`).

Run `proxygen update .` in the project root to update automatically generated exports.

Then build the project.

## Usage

```
A DLL export dumper and proxy generator

Usage: proxygen <COMMAND>

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

## Toolchains and building

You need to build this project with nightly rust since we rely on naked functions.

Note, there is a `toolchain.toml` file that sets the channel to `nightly` in the generated project.

### Building in general

In general, you can just run the following and build the project for whatever default target the project has.

```bash
cargo build --release
```

This will obviously require that you install the correct toolchain, and msys2 for i686 DLLs (see below).

### Building for x86_64

By default, 64-bit DLLs will be built with the `x86_64-pc-windows-msvc` target.

If you'd like to build with the GNU toolchain instead, don't forget to [Install msys2](https://www.msys2.org/), update your path, and install `mingw-w64-x86_64-toolchain`.

Then, just run:

```bash
cargo build --release --target=x86_64-pc-windows-gnu
```

### Building for i686

There are some name mangling issues when building for `i686-pc-windows-msvc`, and you will probably get linking errors.

So I suggest building for `i686-pc-windows-gnu` if you run into issues

#### Setting up to build for `i686-pc-windows-gnu`

Install the `nightly-i686-pc-windows-gnu` toolchain

```bash
rustup toolchain install nightly-i686-pc-windows-gnu
```

Add the `i686-pc-windows-gnu` target:

```bash
rustup target add i686-pc-windows-gnu
```

[Install msys2](https://www.msys2.org/).

Add your msys2/mingw32/bin folder to your system path.

Then, open up a mingw64 terminal, and install `mingw-w64-i686-toolchain`

```bash
pacman -S mingw-w64-i686-toolchain
```

Open a new terminal in your proxy project, and build it

```bash
cargo build --release --target=i686-pc-windows-gnu
```

## Example usage

```bash
proxygen generate path/to/some_library.dll my_some_library_proxy
```

And just like that, you have a ready to compile DLL proxy Rust project.

Then add some exports you want to replace to `intercepted_exports.rs`.

Eg. you could intercept/proxy the `_SomeMangledFunctionName@12` function.
Assuming it has a `void*` (pointer sized type) followed by an `int` (32-bits usually), and returns a boolean

We can proxy it by adding the following to `intercepted_exports.rs`:

```rust
#[no_mangle]
#[export_name = "_SomeMangledFunctionName@12"]
pub unsafe extern "C" fn SomeFunctionName(
    some_arg_1: usize,
    some_arg_2: u32,
) -> bool {
    let original_result: bool = std::mem::transmute::<FARPROC, fn(usize, u32) -> bool>(
        ORIGINAL_FUNCS[Index_SomeFunctionName],
    )(some_arg_1, some_arg_2);
    println!(
        "Proxied SomeFunctionName. Original result: {}. Returning true instead",
        original_result
    );
    true
}
```

And then update your exports by running this in the root of the project before building:
```bash
proxygen update .
```

Build the DLL:

```bash
cargo build --release
```

Next, rename the original DLL and add an underscore to the end.
Copy the dll from the target folder into the same folder as the original DLL.

Run the program and you should see a console appear. Anything you send to stdout or stderr will appear in that console.
