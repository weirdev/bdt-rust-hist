pub mod bstring;
pub mod bint32;
pub mod blist;

use serde_json::Value;

pub trait FromJsonValue<T> {
    fn from_json_value(value: &Value) -> T;
}

pub trait ToRust<'a, R> {
    fn to_rust(&'a self) -> R;
}
