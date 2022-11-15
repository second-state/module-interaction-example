use wasmedge_sdk::error::HostFuncError;
use wasmedge_sdk::host_function;

use wasmedge_sdk::{Caller, WasmValue};

fn load_string(caller: &Caller, addr: u32, size: u32) -> String {
    let mem = caller.memory(0).unwrap();
    let data = mem.read(addr, size).expect("fail");
    String::from_utf8_lossy(&data).to_string()
}

#[host_function]
pub fn host_println(
    caller: Caller,
    input: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    let addr = input[0].to_i32() as u32;
    let size = input[1].to_i32() as u32;
    let s = load_string(&caller, addr, size);
    println!("Rust: `host_println` is printing: \"{}\"", s);

    Ok(vec![])
}
