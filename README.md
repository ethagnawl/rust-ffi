## About
This project demonstrates how you might write a Rust program which uses FFI to
call into shared C libraries (dynamic and static). One library (libyaml) is a
system-level shared library (in /usr/include on Debian 9.9) and the other is a
static library (libgreet) included in this repository.

As usual, [the Rust FFI docs](https://doc.rust-lang.org/nomicon/ffi.html) are excellent and I was able to put this demo
together without much trouble. Though, it wasn't immediately obvious how to
tell cargo/rustc how to find libgreet. I eventually stumbled upon [this](https://old.reddit.com/r/rust/comments/39dckn/rust_ffi_can_i_pass_linker_flags_through_cargo/) Reddit
thread and determined that I could use the `LIBRARY_PATH` env var when calling
`cargo run`. (There may be other or better ways of doing this and I'll make
note of them as I find them.)

This "hello world"/FFI demo is something I've started doing when learning a new
language in order to 1) have an excuse to write a program and 2) learn about
how the language is capable of working with its host environment and existing
tools and utilities. (If interested, see my [ECL](https://github.com/ethagnawl/ecl-hello-r-lisp) and [Crystal](https://github.com/ethagnawl/crystal-c-interop-demo) demos.)

## Run
#### (requires gcc, ar and a version of libyaml exposing the `yaml_get_version_string` function)

`pushd src/lib && make && popd && LIBRARY_PATH=./src/lib cargo run`

## TODO
- build shared library using a [Rust build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
