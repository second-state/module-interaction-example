#![feature(wasm_abi)]

mod host;
use host::HostString;

#[link(wasm_import_module = "host")]
extern "wasm" {
    fn host_println(fmt: HostString);
}

#[no_mangle]
pub unsafe extern "wasm" fn foo(str: HostString) -> HostString {
    host_println(str);
    let json = "[1, 2, 3]";
    json.into()
}
