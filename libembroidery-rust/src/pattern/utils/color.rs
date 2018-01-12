use std::ffi::CStr;
use std::fmt;
use std;

use libc;
use rand;

#[derive(Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct EmbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Display for EmbColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:X}{:X}{:X}", self.r, self.g, self.b)
    }
}

impl Clone for EmbColor {
    fn clone(&self) -> Self {
        *self
    }
}

impl EmbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r: r, g: g, b: b }
    }
    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }
    pub fn random() -> Self {
        Self::new(rand::random(), rand::random(), rand::random())
    }

}

#[no_mangle]
pub extern "C" fn embColor_make(r: u8, g: u8, b: u8) -> EmbColor {
    EmbColor { r: r, g: g, b: b }
}

#[no_mangle]
pub extern "C" fn embColor_create(r: u8, g: u8, b: u8) -> *mut EmbColor {
    println!("WARN: embColor_create is deprecated");
    let heap_color: *mut EmbColor =
        unsafe { libc::malloc(::std::mem::size_of::<EmbColor>()) as (*mut EmbColor) };
    if heap_color.is_null() {
        embLog_error!("emb-color.c embColor_create(), cannot allocate memory for heap_color\n\0");
        0i32 as (*mut EmbColor)
    } else {
        unsafe {
            (*heap_color).r = r;
            (*heap_color).g = g;
            (*heap_color).b = b;
        }
        heap_color
    }
}

#[no_mangle]
pub extern "C" fn embColor_fromHexStr(val: *const std::os::raw::c_char) -> EmbColor {
    let value = unsafe { CStr::from_ptr(val).to_string_lossy().into_owned() };
    EmbColor {
        r: from_hex_or!(value, 0..2),
        g: from_hex_or!(value, 2..4),
        b: from_hex_or!(value, 4..6),
    }
}
