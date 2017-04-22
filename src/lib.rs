use std::slice;
use std::str;

#[no_mangle]
pub extern fn hello_rust() -> *const u8 {
    "Hello world\0".as_ptr()
}

#[no_mangle]
pub extern fn suma(a: i32, b: i32) -> i32 {
    println!("sumando dentro de rust {} + {}", a, b);
    a + b
}

#[repr(C)]
pub struct Dato {
    n: i32,
    cadenac: *const u8,
    cadenarust: String,
    vec: Vec<i32>,
}

#[no_mangle]
pub extern fn dato_crear(n: i32, cadenac: *const u8, l: usize) -> *mut Dato {
    let mut s = unsafe {
        String::from(str::from_utf8(slice::from_raw_parts(cadenac, l)).unwrap())
    };
    s = s + ", desde Rust";
    let v = vec![n, n, n];
    let dato = Dato{n: n, cadenac: cadenac, cadenarust: s, vec: v};

    Box::into_raw(Box::new(dato))
}

#[no_mangle]
pub extern fn dato_print(dato: *mut Dato) {
    unsafe {
        println!("cadena: {}", (*dato).cadenarust);
        println!("vec: {:?}", (*dato).vec);
    }
}

#[no_mangle]
pub extern fn dato_destruir(dato: *mut Dato) {
    unsafe {
        let _ = Box::from_raw(dato);
    }
}
