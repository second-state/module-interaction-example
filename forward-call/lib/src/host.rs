#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct HostString(*const u8, usize);
impl From<&str> for HostString {
    fn from(value: &str) -> Self {
        Self(value.as_ptr(), value.len())
    }
}
impl HostString {
    pub fn len(&self) -> usize {
        self.1
    }
}
