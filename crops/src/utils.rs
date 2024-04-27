pub type CResult = Result<(), String>;

#[repr(C)]
pub struct StringBuffer {
    pub(crate) buffer: *mut libc::c_char,
    pub(crate) len: usize,
}

pub fn check_null<'a, T>(value: *mut T) -> Result<&'a mut T, String> {
    if value.is_null() {
        Err("Null Pointed Received by Rust".into())
    } else {
        Ok(unsafe { &mut *value })
    }
}

pub fn check_null_const<'a, T>(value: *const T) -> Result<&'a T, String> {
    if value.is_null() {
        Err("Null Pointed Received by Rust".into())
    } else {
        Ok(unsafe { &*value })
    }
}

pub fn as_string(c_str: *const libc::c_char) -> Result<String, String> {
    let res = unsafe { std::ffi::CStr::from_ptr(c_str) }
        .to_str()
        .map_err(|e| format!("{e:?}"))
        .map(|s| s.to_string());

    res
}

pub fn copy_string(c_value: StringBuffer, value: &str) -> Result<(), String> {
    let res = std::ffi::CString::new(value)
        .map(|s| s.as_bytes_with_nul().to_vec())
        .map_err(|e| format!("{e:?}"))?;

    let res = unsafe { &*(res.as_slice() as *const [u8] as *const [i8]) };

    let bytes = unsafe { std::slice::from_raw_parts_mut(c_value.buffer, c_value.len) };

    let len = std::cmp::min(res.len(), c_value.len);

    bytes[..len].copy_from_slice(&res[..len]);

    Ok(())
}
