mod bint;
mod spec;

use bint::bstring::BString;
use std::ptr::write_unaligned;

use spec::Spec;

use crate::bint::{bint32::BInt32, blist::BList, bmap::BMap, FromJsonValue, ToRust};

#[no_mangle]
pub extern "C" fn get_bstring1(bstring_data: *mut u8) {
    unsafe {
        write_unaligned(
            bstring_data as *mut BString,
            BString::new("Awesome string from rust!"),
        );
    }
}
