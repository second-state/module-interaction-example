#![feature(wasm_abi)]

#[link(wasm_import_module = "host")]
extern "wasm" {
    fn forward_call(
        module_name: *const u8,
        module_name_len: usize,
        funciton_name: *const u8,
        funciton_name_len: usize,
    ) -> u32;
}

#[no_mangle]
pub fn start() -> u32 {
    let v = unsafe { forward_call("lib".as_ptr(), "lib".len(), "foo".as_ptr(), "foo".len()) };
    v
}
