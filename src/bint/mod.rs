pub mod bint32;
pub mod blist;
pub mod bmap;
pub mod bstring;

use serde_json::Value;

pub trait FromJsonValue<T> {
    fn from_json_value(value: &Value) -> T;
}

pub trait ToRust<'a, R> {
    fn to_rust(&'a self) -> R;
}
