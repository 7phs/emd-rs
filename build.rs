extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/emd/emd.c")
        .flag("-funroll-loops")
        .compile("emd");   // outputs `libhello.a`
}