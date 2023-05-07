use serde_json::Value;
use std::str;

use super::FromJsonValue;

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

impl BString {
    #[inline]
    pub fn to_rust(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.data.as_ref()) }
    }
}
