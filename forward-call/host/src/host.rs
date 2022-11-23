use wasmedge_sdk::Caller;

pub fn load_string(caller: &Caller, addr: u32, size: u32) -> String {
    let mem = caller.memory(0).unwrap();
    let data = mem.read(addr, size).expect("fail to get string");
    String::from_utf8_lossy(&data).to_string()
}
