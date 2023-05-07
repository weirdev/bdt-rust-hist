use serde_json::Value;
use std::str;

use super::{FromJsonValue, ToRust};

pub struct BString {
    data: Box<[u8]>,
}

impl FromJsonValue<BString> for BString {
    fn from_json_value(value: &Value) -> BString {
        BString {
            data: if let Value::String(s) = value {
                s.as_bytes().iter().cloned().collect()
            } else {
                panic!("Attempting to create BString from non-string");
            },
        }
    }
}

impl<'a> ToRust<'a, &'a str> for BString {
    fn to_rust(&'a self) -> &'a str {
        unsafe { str::from_utf8_unchecked(self.data.as_ref()) }
    }
}
