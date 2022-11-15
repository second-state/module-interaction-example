#![feature(wasm_abi)]

#[link(wasm_import_module = "host")]
extern "wasm" {
    fn host_println(ptr: *const u8, size: usize);
}

#[no_mangle]
pub unsafe extern "wasm" fn foo(ptr: *mut u8, size: usize) -> (*const u8, usize) {
    host_println(ptr, size);
    let json = "[1, 2, 3]";
    (json.as_ptr(), json.len())
}
