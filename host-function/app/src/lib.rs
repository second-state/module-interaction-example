extern "C" {
    fn real_add(l: u32, r: u32) -> u32;
    fn real_println(str_ptr: *const u8, str_len: usize) -> ();
}

#[no_mangle]
pub fn start() -> u32 {
    let s = "hello";
    unsafe { real_println(s.as_ptr(), s.len()) };
    unsafe { real_add(1, 2) }
}
