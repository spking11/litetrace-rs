use crate::{
    errors::{Error, Result},
    tracefs_sys as ffi,
};
use std::ffi::{CStr, CString};

pub fn use_tracing_file(name: &str, func: fn(&str) -> Result<()>) -> Result<()> {
    unsafe {
        let cname = CString::new(name).unwrap();
        let path = ffi::tracefs_get_tracing_file(cname.as_ptr() as *const i8);
        if path.is_null() {
            return Err(Error::Nullptr {
                from: "tracefs_get_tracing_file".to_string(),
                args: name.to_string(),
            });
        }
        func(CStr::from_ptr(path).to_str()?)?;
        ffi::tracefs_put_tracing_file(path);
    }
    Ok(())
}
