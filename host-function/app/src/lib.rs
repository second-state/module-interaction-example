#![feature(wasm_abi)]

#[repr(C)]
struct HostString {
    ptr: *mut u8,
    size: usize,
}

#[link(wasm_import_module = "host")]
extern "wasm" {
    fn host_add(l: u32, r: u32) -> u32;
    fn host_println(str_ptr: *const u8, str_len: usize) -> ();
    fn host_suffix(str_ptr: *const u8, str_len: usize) -> HostString;
}

#[no_mangle]
pub fn start() -> u32 {
    let s = "hello";
    unsafe {
        host_println(s.as_ptr(), s.len());
    }
    let s2 = unsafe {
        let HostString { ptr, size } = host_suffix(s.as_ptr(), s.len());
        String::from_raw_parts(ptr, size, size)
    };
    unsafe {
        host_println(s2.as_ptr(), s2.len());
    }
    unsafe { host_add(1, 2) }
}
