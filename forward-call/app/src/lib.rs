#![feature(wasm_abi)]

mod host;
use host::HostString;

#[link(wasm_import_module = "host")]
extern "wasm" {
    fn forward_call(
        module_name: HostString,
        funciton_name: HostString,
        data: HostString,
    ) -> HostString;
    fn host_println(fmt: HostString);
}

#[no_mangle]
pub unsafe fn start() -> u32 {
    let json = "{}";
    let host_str = forward_call("lib".into(), "foo".into(), json.into());
    host_println(host_str.clone());
    host_str.len() as u32
}
