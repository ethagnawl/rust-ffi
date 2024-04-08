// Copied mostly verbatim from:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html

fn main() {
    cc::Build::new().file("src/lib/greet.c").compile("greet");
    println!("cargo:rerun-if-changed=src/lib/greet.c");
}
