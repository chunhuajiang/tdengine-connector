#[path = "bindings.rs"]
mod bindings;
use bindings::*;

use std::os::raw::{c_void, c_char, c_int};
use std::ffi::{CString, CStr};

pub enum Field {
    tinyInt(i8),
    smallInt(i16),
    normalInt(i32),
    bigInt(i64),
    float(f32),
    double(f64),
    binary(String),
    timeStamp(i64),
    boolType(bool),
}

pub type Fields = Vec<Field>;
pub type Row = Vec<Fields>;

pub fn str_into_raw(s: &str) -> *mut c_char {
    if s.is_empty() {
        0 as *mut c_char
    } else {
        CString::new(s).unwrap().into_raw()
    }
}

pub fn raw_into_str<'a>(raw: *mut c_char) -> &'static str {
    unsafe {CStr::from_ptr(raw).to_str().unwrap()}
}
