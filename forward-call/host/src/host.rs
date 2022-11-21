use wasmedge_sdk::CallingFrame;

pub fn load_string(frame: &CallingFrame, addr: u32, size: u32) -> String {
    let mem = frame.memory_mut(0).unwrap();
    let data = mem.get_data(addr, size).expect("fail to get string");
    String::from_utf8_lossy(&data).to_string()
}
