# C-Rust (inter)Operability Kit

An in-development workspace for Rust-C Interoperability Tooling

## How does it work?

Consider the `simple` example, outline in [`examples/simple`](examples/simple).

The rust code is very simple, and can be found [`lib.rs`](examples/simple/src/lib.rs)

The derive macros applied to the struct and to the enums will produce code that can be seen in [`expanded.rs`](examples/simple/expanded.rs).
Notice that the expanded code has numerous `#[no_mangle] pub extern "C"` functions. This is the entire point!

We've generated functions to create, debug, mutate, and get data on the enums and structs that we created.

This effectively creates a safe FFI, where rather than passing struct's data across the barrier, we are always only passing pointers, or raw clib data types.

You'll also notice a nested function, that is not public. The outer function that is `extern "C"` returns a C error code, through the convention of an `int`. But the inner function is rust function, that uses the `utils::CResult` to use semantically meaninful rust to return a rust Result type.

## Generated Interfaces

All of these will return an error code if you pass in a null pointer

- `Vec`
    - `push`: Push an element to the end of the vector (rust creates a Clone of your data structure)
    - `get`: Get a copy of an element at index `idx` - we return a copy so you don't accidentaly leave rust with an invalid pointer in it's vector.
        - Will return error code if `idx` is out of range
    - `remove`: Remove and get an element at index `idx`
        - Will return error code is `idx` is out of range 
- `Option`
    - `get`: Get a copy to to the value inside the option
        - Will return error code is `Option::is_none`
    - `replace`: Replace the value in the option
    - `take`: Take and return the value inside the option.
        - Will return error code if `Option::is_none`

## Generating C

We then use `cbindgen` to build a c-api based on these `#[no_mangle]` functions, which can be sen in the [`simple.h`](examples/simple/include/simple.h).

The proccess for this is done in `flake.nix`, but could also be done in a `build.rs` or however you prefer to use `cbindgen`.

## Generated Python

It is also now easy to create a python interface to our rust structures. With [`ctypesgen`](https://github.com/ctypesgen/ctypesgen), we can quickly and easily generate python bindings to a c library using it's header files.

We do this in the `flake.nix` `mkExample` function as well. This is the easiest way to demo this functionality. The API you'll see in the python repl will be *almost* identical to that of the C api.

## Demo

Want to try it out?

1. First `nix develop` in the root.

2. The `simple` example will be available to play with in the python repl.
```bash
$ nix develop
$ python
>>> import simple
>>> b = simple.brush_default()
>>> simple.brush_debug(b)
```
