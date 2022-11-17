#![feature(wasm_abi)]

mod host;
use common::Person;
use host::HostString;

#[link(wasm_import_module = "host")]
extern "wasm" {
    fn host_println(fmt: HostString);
}

#[no_mangle]
pub unsafe extern "wasm" fn growup(str: HostString) -> HostString {
    host_println(str.clone());
    let mut person: Person = serde_json::from_str(Into::<String>::into(str).as_str()).unwrap();
    person.age += 3;
    let json = serde_json::to_string(&person).unwrap();
    json.as_str().into()
}
