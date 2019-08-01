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


            // for out in &outs {
            //     println!("type: {}, bytes: {}", out.type_, out.bytes);
            // }

            Ok(Subscriber{tsub, fields, fcount})
        }
    }

    pub fn consume(self: &Subscriber) {
        unsafe {
            let taosRow = taos_consume(self.tsub);
            if taosRow.is_null() {
                panic!("taos_consume error");
            }
            self.raw_into_row(taosRow);
        }
    }

    pub fn raw_into_row(self: &Subscriber, row: *mut *mut c_void) {
        // let rows: Vec<Fields> = Vec::new();
        let fields = raw_into_field(self.fields, self.fcount);

        for (i, field) in fields.iter().enumerate() {
            print!("index: {}, type: {}, bytes: {}", i, field.type_, field.bytes);
            match field.type_ as u32 {
                TSDB_DATA_TYPE_TINYINT => {
                    println!("{} ", unsafe {*(row.offset(i as isize) as *mut i8)});
                }
                TSDB_DATA_TYPE_SMALLINT => {
                    println!("{} ", unsafe {*(row.offset(i as isize) as *mut i16)});
                }
                TSDB_DATA_TYPE_INT => {
                    println!("{} ", unsafe {*(row.offset(i as isize) as *mut u32)});
                }
                TSDB_DATA_TYPE_BIGINT => {
                    println!("{} ", unsafe {*(row.offset(i as isize) as *mut i64)});
                }
                TSDB_DATA_TYPE_FLOAT => {
                    println!("{} ", unsafe {*(row.offset(i as isize) as *mut f32)});
                }
                TSDB_DATA_TYPE_DOUBLE => {
                    println!("{} ", unsafe {*(row.offset(i as isize) as *mut f64)});
                }
                TSDB_DATA_TYPE_BINARY | TSDB_DATA_TYPE_NCHAR => {
                    // TODO
                    println!("{} ", unsafe {*(row.offset(i as isize) as *mut i8)});
                }
                TSDB_DATA_TYPE_TIMESTAMP => {
                    println!("{} ", unsafe {*(row.offset(i as isize) as *mut i64)});
                }
                TSDB_DATA_TYPE_BOOL => {
                    println!("{} ", unsafe {*(row.offset(i as isize) as *mut i8)});
                }
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
    let range = std::ops::Range{start: 0, end: fcount};

    for i in range {
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