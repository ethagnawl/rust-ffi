## About
This project demonstrates how you might write a Rust program which uses Rust's
FFI to call into a static C library.

As usual, [the Rust FFI docs](https://doc.rust-lang.org/nomicon/ffi.html) are excellent and I was able to put this demo
together without much trouble. Though, it wasn't immediately obvious how to
tell cargo/rustc how to find my library. I eventually stumbled upon this Reddit
thread and determined that I could use the `LIBRARY_PATH` env var when calling
`cargo run`. (There may be other or better ways of doing this and I'll make note
of them as I find them.)

## Run
`LIBRARY_PATH=./src/lib cargo run`
