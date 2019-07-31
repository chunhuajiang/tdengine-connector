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
        unsafe {
            let taosRow = taos_consume(self.tsub);
        }
    }
}


impl Drop for Subscriber {
    fn drop(&mut self) {
        println!("DROP FROM Subscriber");
        unsafe {taos_unsubscribe(self.tsub);}
    }
}