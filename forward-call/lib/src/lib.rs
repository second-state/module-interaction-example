#![feature(wasm_abi)]

#[link(wasm_import_module = "host")]
extern "wasm" {
    fn host_println(str: *const u8, str_len: usize);
}

#[no_mangle]
pub fn foo() -> u32 {
    let s = "test";
    unsafe {
        host_println(s.as_ptr(), s.len());
    }
    100
}
