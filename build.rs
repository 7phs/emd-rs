extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/emd/emd.c")
        .compile("emd");   // outputs `libhello.a`
}