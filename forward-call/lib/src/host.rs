#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct HostString(*const u8, usize);

impl From<&str> for HostString {
    fn from(value: &str) -> Self {
        Self(value.as_ptr(), value.len())
    }
}

impl Into<String> for HostString {
    fn into(self) -> String {
        let v = unsafe { Vec::from_raw_parts(self.0 as *mut u8, self.1, self.1) };
        String::from_utf8_lossy(&v).to_string()
    }
}

impl HostString {
    pub fn len(&self) -> usize {
        self.1
    }
}
