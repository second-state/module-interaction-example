#![feature(wasm_abi)]

#[link(wasm_import_module = "host")]
extern "wasm" {
    fn hello();

}

#[no_mangle]
pub fn foo() {
    unsafe { hello() }
}
