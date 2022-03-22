use crate::{
    errors::{Error, Result},
    tracefs_sys as ffi,
};
use std::{
    ffi::{CStr, CString},
    ptr,
};

pub fn get_tracing_file(name: &str) -> Result<String> {
    let result: String;
    unsafe {
        let cname = CString::new(name).unwrap();
        let path = ffi::tracefs_get_tracing_file(cname.as_ptr() as *const i8);
        if path.is_null() {
            return Err(Error::Nullptr {
                from: "tracefs_get_tracing_file".to_string(),
                args: name.to_string(),
            });
        }
        result = CStr::from_ptr(path).to_str()?.to_owned();
        ffi::tracefs_put_tracing_file(path);
    }
    Ok(result)
}

pub struct Instance {
    tracefs: *mut ffi::tracefs_instance,
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            ffi::tracefs_instance_free(self.tracefs);
        }
    }
}

static mut TOP_INSTANCE: Option<&'static mut Instance> = None;

impl Instance {
    pub fn new(name: &str) -> Instance {
        return Instance {
            tracefs: unsafe {
                let cname = CString::new(name).unwrap();
                ffi::tracefs_instance_create(cname.as_ptr() as *const i8)
            },
        };
    }

    pub fn top() -> &'static mut Instance {
        unsafe {
            if TOP_INSTANCE.is_none() {
                let ins = Box::new(Instance {
                    tracefs:  ffi::tracefs_instance_create(ptr::null() as *const i8) ,
                });
                TOP_INSTANCE = Option::Some(Box::leak(ins));
            }
            return TOP_INSTANCE.as_mut().unwrap();
        }
    }
}

pub fn instance_get_file(ins: &Instance, name: &str) -> Result<String> {
    let result: String;
    unsafe {
        let cname = CString::new(name).unwrap();
        let path = ffi::tracefs_instance_get_file(ins.tracefs, cname.as_ptr() as *const i8);
        if path.is_null() {
            return Err(Error::Nullptr {
                from: "tracefs_instance_get_file".to_string(),
                args: name.to_string(),
            });
        }
        result = CStr::from_ptr(path).clone().to_str()?.to_owned();
        ffi::tracefs_put_tracing_file(path);
    }
    Ok(result)
}
