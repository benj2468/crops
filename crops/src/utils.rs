pub type CResult = Result<(), String>;

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

pub fn copy_string(c_value: *mut libc::c_char, value: &str) -> Result<(), String> {
    let res = std::ffi::CString::new(value)
        .map(|s| s.as_bytes_with_nul().to_vec())
        .map_err(|e| format!("{e:?}"))?;

    let res = &*(res.as_slice());

    let bytes = unsafe { std::slice::from_raw_parts_mut(c_value as *mut u8, value.len()) };

    bytes[..value.len()].copy_from_slice(&res[..value.len()]);

    Ok(())
}

pub mod duration {
    pub type Duration = std::time::Duration;
    #[no_mangle]
    pub extern "C" fn duration_from_ms(ms: u64) -> *mut Duration {
        Box::into_raw(Box::new(std::time::Duration::from_millis(ms)))
    }
}
