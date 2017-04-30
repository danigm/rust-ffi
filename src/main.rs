extern crate libc;

#[link(name = "cffi")]
extern {
    fn repeat(s: *const u8, n: i32);
}

fn safe_repeat(s: &str, n: i32) {
    let st = String::from(s) + "\0";
    unsafe {
        repeat(st.as_ptr(), n);
    }
}

fn main() {
    unsafe {
        repeat("Salida desde C\0".as_ptr(), 5);
    }

    safe_repeat("Safe", 10);
}
