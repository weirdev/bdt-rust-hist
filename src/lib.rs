#![feature(generic_const_exprs)]

mod bint;
mod spec;

use bint::bstring::BString;
use std::ptr::write_unaligned;

use spec::Spec;

use crate::bint::{bint32::BInt32, blist::BList, bmap::BMap, FromJsonValue, ToRust};

fn list_string1() -> BList<BString> {
    let spec = Spec::read_from_file("../json-specs/spec4.json").unwrap();

    BList::<BString>::from_json_value(&spec.value)
}

fn map_string_int1() -> BMap<BString, BInt32> {
    let spec = Spec::read_from_file("../json-specs/spec5.json").unwrap();

    BMap::<BString, BInt32>::from_json_value(&spec.value)
}

unsafe fn write_to_c<T>(target_ptr: *mut u8, src: T) {
    // TODO: should this just be std::ptr::write?
    write_unaligned(target_ptr as *mut T, src);
}

#[no_mangle]
pub extern "C" fn get_bstring1(bstring_data: *mut u8) {
    unsafe {
        write_to_c(bstring_data, BString::new("Awesome string from rust!"));
    }
}

#[no_mangle]
pub extern "C" fn get_blist_string1(blist_data: *mut u8) {
    unsafe {
        write_to_c(blist_data, list_string1());
    }
}

#[no_mangle]
pub extern "C" fn get_bmap_string_int1(blist_data: *mut u8) {
    let map = map_string_int1();
    unsafe {
        write_to_c(blist_data, map);
    }
}
