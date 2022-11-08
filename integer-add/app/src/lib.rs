#[link(wasm_import_module = "module")]
extern "C" {
    fn add(left: i32, right: i32) -> i32;
}

#[no_mangle]
pub unsafe fn start() -> i32 {
    add(1, 2)
}
