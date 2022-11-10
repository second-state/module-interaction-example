extern "C" {
    fn real_add(l: u32, r: u32) -> u32;
}

#[no_mangle]
pub fn start() -> u32 {
    unsafe { real_add(1, 2) }
}
