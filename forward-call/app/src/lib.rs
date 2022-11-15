#![feature(wasm_abi)]

#[link(wasm_import_module = "host")]
extern "wasm" {
    fn forward_call(
        module_name: *const u8,
        module_name_len: usize,
        funciton_name: *const u8,
        funciton_name_len: usize,
        arg: *const u8,
        arg_len: usize,
    ) -> (*const u8, usize);
    fn host_println(ptr: *const u8, size: usize);
}

#[no_mangle]
pub unsafe fn start() -> u32 {
    let json = "{}";
    let (ptr, size) = forward_call(
        "lib".as_ptr(),
        "lib".len(),
        "foo".as_ptr(),
        "foo".len(),
        json.as_ptr(),
        json.len(),
    );
    host_println(ptr, size);
    size as u32
}
