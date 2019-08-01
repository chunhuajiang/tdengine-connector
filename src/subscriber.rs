#[path = "bindings.rs"]
mod bindings;
use bindings::*;

#[path = "utils.rs"]
mod utils;
use utils::*;

use std::os::raw::c_void;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_long;

pub struct Subscriber {
    tsub: *mut c_void,
    fields: *mut taosField,
    fcount: c_int,
}

impl Subscriber {
    pub fn new(host: &str,
               username: &str,
               passwd: &str,
               db: &str,
               table:&str,
               time: i64,
               mseconds: i32
              ) -> Result<Subscriber, &'static str> {
        unsafe {
            let mut tsub = taos_subscribe(str_into_raw(host),
                                          str_into_raw(username),
                                          str_into_raw(passwd),
                                          str_into_raw(db),
                                          str_into_raw(table),
                                          time as c_long,
                                          mseconds as c_int);
            if tsub.is_null() {
                return Err("subscribe error")
            }
            println!("subscribed to {} user:{}, db:{}, tb:{}, time:{}, mseconds:{}",
                        host, username, db, table, time, mseconds);

            let mut fields = taos_fetch_subfields(tsub);
            if fields.is_null() {
                taos_unsubscribe(tsub);
                return Err("fetch subfields error")
            }

            let fcount = taos_subfields_count(tsub);
            if fcount == 0 {
                taos_unsubscribe(tsub);
                return Err("subfields count is 0")
            }

            Ok(Subscriber{tsub, fields, fcount})
        }
    }

    pub fn consume(self: &Subscriber) {
        println!("hahaha");
        unsafe {
            let taosRow = taos_consume(self.tsub);
            println!("aaa");
            if taosRow.is_null() {
                panic!("taos_consume error");
            }

            // // allocate and zero memory for the object
            // let ffi_obj = FfiObject::new(256);
            // // Pass the memory to a foreign function
            // unsafe {
            //     taos_print_row(ffi_obj.ptr, taosRow, self.fields, self.fcount);
            //     println!("print row: {}", raw_into_str(ffi_obj.ptr));
            // }

            let taosRow: Vec<*mut c_void> = Vec::from_raw_parts(taosRow, self.fcount as usize, 10);
            // // println!("raw address: {}", taosRow as i32);
            self.raw_into_row(&taosRow);
        }
        println!("hahahb");
    }

    pub fn raw_into_row(self: &Subscriber, row: & Vec<*mut c_void>) {
        // let rows: Vec<Fields> = Vec::new();
        let fields = raw_into_field(self.fields, self.fcount);

        for (i, field) in fields.iter().enumerate() {
            print!("index: {}, type: {}, bytes: {}", i, field.type_, field.bytes);
            match field.type_ as u32 {
                // TSDB_DATA_TYPE_TINYINT => {
                //     println!("{} ", unsafe {*(row.offset(i as isize) as *mut i8)});
                // }
                // TSDB_DATA_TYPE_SMALLINT => {
                //     println!("{} ", unsafe {*(row.offset(i as isize) as *mut i16)});
                // }
                TSDB_DATA_TYPE_INT => {
                    // println!("{} ", unsafe {*(row.offset(i as isize) as *mut i32)});
                    println!("  {} ", unsafe {*(row[i] as *mut i32)});
                }
                // TSDB_DATA_TYPE_BIGINT => {
                //     println!("{} ", unsafe {*(row.offset(i as isize) as *mut i64)});
                // }
                // TSDB_DATA_TYPE_FLOAT => {
                //     println!("{} ", unsafe {*(row.offset(i as isize) as *mut f32)});
                // }
                // TSDB_DATA_TYPE_DOUBLE => {
                //     println!("{} ", unsafe {*(row.offset(i as isize) as *mut f64)});
                // }
                // TSDB_DATA_TYPE_BINARY | TSDB_DATA_TYPE_NCHAR => {
                //     // TODO
                //     println!("{} ", unsafe {*(row.offset(i as isize) as *mut i8)});
                // }
                TSDB_DATA_TYPE_TIMESTAMP => {
                    // println!(" int {} ", unsafe {*((*(row as *mut u32) as *mut i64).offset(i as isize) as *mut i32)});
                    // println!(" timestamp: {} ", unsafe {*((row as *mut i64).offset(i as isize))});
                    println!("  {} ", unsafe {*(row[i] as *mut i64)});
                }
                // TSDB_DATA_TYPE_BOOL => {
                //     println!("{} ", unsafe {*(row.offset(i as isize) as *mut i8)});
                // }
                _ => println!(""),
            }
        }
    }
}


impl Drop for Subscriber {
    fn drop(&mut self) {
        println!("DROP FROM Subscriber");
        unsafe {taos_unsubscribe(self.tsub);}
    }
}

pub fn raw_into_field(raw: *mut TAOS_FIELD, fcount: c_int) -> Vec<taosField> {
    let mut fields: Vec<taosField> = Vec::new();

    for i in 0..fcount as isize {
        fields.push(
            taosField {
                name: unsafe {(*raw.offset(i as isize))}.name,
                bytes: unsafe {(*raw.offset(i as isize))}.bytes,
                type_: unsafe {(*raw.offset(i as isize))}.type_,
            }
        );
    }
    /// TODO: error[E0382]: use of moved value: `fields`
    // for field in &fields {
    //     println!("type: {}, bytes: {}", field.type_, field.bytes);
    // }

    fields
}

// pub struct FfiObject {
//     pub ptr: *mut i8,
//     pub size: usize,
// }

// impl FfiObject {
//     // allocate and zero memory
//     pub fn new(size: usize) -> FfiObject {
//         FfiObject::_from_vec(vec![0i8; size], size)
//     }
//     // allocate memory without zeroing
//     pub fn new_uninitialized(size: usize) -> FfiObject {
//         FfiObject::_from_vec(Vec::with_capacity(size), size)
//     }
//     fn _from_vec(mut v: Vec<i8>, size: usize) -> FfiObject {
//         assert!(size > 0);
//         let ptr = v.as_mut_ptr();
//         std::mem::forget(v);
//         FfiObject { ptr, size }
//     }
// }
// impl Drop for FfiObject {
//     fn drop(&mut self) {
//         unsafe { std::mem::drop(Vec::from_raw_parts(self.ptr, 0, self.size)) };
//     }
// }