use serde_json::Value;
use std::mem::transmute;
use std::slice;
use std::str;

use super::{FromJsonValue, ToRust};

#[repr(transparent)]
pub struct BString {
    data: [u8; 16],
}

impl BString {
    pub fn new(s: &str) -> BString {
        if s.len() > 0 {
            let boxed: Box<[u8]> = s.as_bytes().iter().cloned().collect();

            let fat_ptr = Box::into_raw(boxed);
            let raw_ptr = fat_ptr as *mut u8 as usize;

            BString {
                data: unsafe { transmute([raw_ptr, s.len()]) },
            }
        } else {
            BString {
                data: unsafe { transmute([0 as u64, 0 as u64]) },
            }
        }
    }

    fn get_elements_as_slice(&self) -> &[u8] {
        let raw_ptr = usize::from_le_bytes(self.data[..8].try_into().unwrap()) as *const u8;
        let len = usize::from_le_bytes(self.data[8..].try_into().unwrap());

        // println!("raw_ptr: {:p}", raw_ptr);
        // println!("len: {}", len);

        unsafe { slice::from_raw_parts(raw_ptr, len) }
    }

    fn get_elements_as_slice_mut(&mut self) -> &mut [u8] {
        let raw_ptr = usize::from_le_bytes(self.data[..8].try_into().unwrap()) as *mut u8;
        let len = usize::from_le_bytes(self.data[8..].try_into().unwrap());

        // println!("raw_ptr: {:p}", raw_ptr);
        // println!("len: {}", len);

        unsafe { slice::from_raw_parts_mut(raw_ptr, len) }
    }
}

impl FromJsonValue<BString> for BString {
    fn from_json_value(value: &Value) -> BString {
        if let Value::String(s) = value {
            BString::new(s)
        } else {
            panic!("Attempting to create BString from non-string");
        }
    }
}

impl<'a> ToRust<'a, &'a str> for BString {
    fn to_rust(&'a self) -> &'a str {
        unsafe { str::from_utf8_unchecked(self.get_elements_as_slice()) }
    }
}

impl Drop for BString {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.get_elements_as_slice_mut() as *mut [u8]) };
    }
}
