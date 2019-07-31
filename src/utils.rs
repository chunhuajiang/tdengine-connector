use std::os::raw::c_void;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ffi::CString;
use std::ffi::CStr;

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

type Fields = Vec<Field>;
type Row = Vec<Fields>;

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

pub fn raw_into_field(taosFields: *mut c_char, fcount: c_int)/* -> Fields */{
    let mut fields: Vec<Field>= Vec::new();
    let mut rawField: taosField= taosFields;

    let i = 1;
    loop {
        if (i > fcount) {
            break;
        }



    }
}

// pub fn raw_into_row(raw: *mut c_char, int ) -> Row {
    
// }