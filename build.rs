extern crate gcc;

fn main() {
    gcc::compile_library("libcffi.a", &["src/lib.c"]);
}
