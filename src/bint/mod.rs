pub mod bstring;
pub mod bint32;

use serde_json::Value;

pub trait FromJsonValue<T> {
    fn from_json_value(value: &Value) -> T;
}