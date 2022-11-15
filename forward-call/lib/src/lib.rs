#![feature(wasm_abi)]

#[no_mangle]
pub extern "wasm" fn foo(ptr: *mut u8, size: usize) -> (*const u8, usize) {
    let json = "[1, 2, 3]";
    (json.as_ptr(), json.len())
}
