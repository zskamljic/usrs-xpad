mod keys;

pub use keys::{Key, Side};
use std::ffi::CString;
use std::io::{Error, ErrorKind, Result};
use std::os::raw::{c_char, c_int, c_short, c_ushort};

use libc::{O_NONBLOCK, O_WRONLY};

extern "C" {
    fn close_and_destroy(file: c_int);
    fn setup_device(file: c_int, name: *const c_char, vendor_id: c_short, product_id: c_short);
    fn set_key(file: c_int, key: u32, pressed: bool);
    fn set_axis(file: c_int, side: u32, x: c_short, y: c_short, z: c_ushort);
}

pub struct UInputHandle {
    file: c_int,
}

impl UInputHandle {
    pub fn new(name: &str, vendor_id: u16, product_id: u16) -> Result<UInputHandle> {
        let handle = match open_uinput() {
            Some(value) => value,
            None => return Err(Error::new(ErrorKind::NotFound, "No uinput found")),
        };
        let name = CString::new(name).unwrap();
        unsafe {
            setup_device(handle, name.as_ptr(), vendor_id as i16, product_id as i16);
        }
        Ok(UInputHandle { file: handle })
    }

    pub fn set_key_pressed(&self, key: Key, pressed: bool) {
        unsafe {
            set_key(self.file, key as u32, pressed);
        }
    }

    pub fn update_axis(&self, side: Side, x: i16, y: i16, z: u16) {
        unsafe {
            set_axis(self.file, side as u32, x, y, z);
        }
    }
}

impl Drop for UInputHandle {
    fn drop(&mut self) {
        unsafe {
            close_and_destroy(self.file);
        }
    }
}

fn open_uinput() -> Option<c_int> {
    let candidate_paths = [
        CString::new("/dev/uinput").unwrap(),
        CString::new("/dev/input/uinput").unwrap(),
    ];
    for path in candidate_paths.iter() {
        let path = path.as_ptr();
        let handle = unsafe { libc::open(path, O_WRONLY | O_NONBLOCK) };
        if handle > 0 {
            return Some(handle);
        }
    }
    None
}
