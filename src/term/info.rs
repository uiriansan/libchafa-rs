use crate::ffi;

pub struct TermInfo {
    pub raw: *mut ffi::ChafaTermInfo,
}
