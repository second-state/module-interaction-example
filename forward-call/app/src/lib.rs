#![feature(wasm_abi)]

mod host;
use host::HostString;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

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
    let person = Person {
        name: "August".into(),
        age: 16,
    };
    let json = serde_json::to_string(&person).unwrap();
    let host_str = forward_call("lib".into(), "growup".into(), json.as_str().into());
    host_println(host_str.clone());
    host_str.len() as u32
}
