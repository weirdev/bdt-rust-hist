use serde_json::Value;

use super::{buint32::BUInt32, FromJsonValue, ToRust};

pub struct BList<T> {
    length: BUInt32,
    data: Box<[T]>,
}

impl<T: FromJsonValue<T>> FromJsonValue<BList<T>> for BList<T> {
    fn from_json_value(value: &serde_json::Value) -> BList<T> {
        if let Value::Array(values) = value {
            return BList {
                length: BUInt32 {
                    data: (values.len() as u32).to_le_bytes(),
                },
                data: values.iter().map(|v| T::from_json_value(v)).collect(),
            };
        }
        panic!("Attempting to construct BList from non-array")
    }
}

impl<'a, T> ToRust<'a, &'a [T]> for BList<T> {
    fn to_rust(&'a self) -> &'a [T] {
        self.data.as_ref()
    }
}
