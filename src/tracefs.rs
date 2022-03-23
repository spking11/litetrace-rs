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
                    tracefs: ffi::tracefs_instance_create(ptr::null() as *const i8),
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

// pub fn instance_file_read(ins: &Instance, file: &str) -> Result<String> {
//     let result = unsafe {
//         let cfile = CString::new(file).unwrap();
//         let buf = ffi::tracefs_instance_file_read(
//             ins.tracefs,
//             cfile.as_ptr() as *const i8,
//             ptr::null_mut() as *mut i32,
//         );
//         if buf.is_null() {
//             return Err(Error::Nullptr {
//                 from: "tracefs_instance_file_read".to_string(),
//                 args: file.to_string(),
//             });
//         }
//         CStr::from_ptr(buf).clone().to_str()?.to_owned()
//     };
// }

pub use crate::tracefs_sys::TRACEFS_FL_CONTINUE;
pub use crate::tracefs_sys::TRACEFS_FL_FUTURE;
pub use crate::tracefs_sys::TRACEFS_FL_RESET;

pub fn function_filter(
    ins: &Instance,
    filter: Option<String>,
    module: Option<String>,
    flags: u32,
) -> Result<()> {
    unsafe {
        let cfilter: CString;
        let cmodule: CString;
        let ret = ffi::tracefs_function_filter(
            ins.tracefs,
            if let Some(filter) = filter {
                cfilter = CString::new(filter).unwrap();
                cfilter.as_ptr() as *const i8
            } else {
                ptr::null() as *const i8
            },
            if let Some(module) = module {
                cmodule = CString::new(module).unwrap();
                cmodule.as_ptr() as *const i8
            } else {
                ptr::null() as *const i8
            },
            flags,
        );
        if ret < 0 {
            return Err(Error::CLibError {
                func: "tracefs_function_filter".to_string(),
                msg: "".to_string(),
            });
        }
    }
    Ok(())
}
