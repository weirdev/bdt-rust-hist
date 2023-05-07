use serde_json::Value;

use crate::bint::buint32::BUInt32;

use super::{FromJsonValue, ToRust};

pub struct BMap<K, V> {
    length: BUInt32,
    // TODO: Tree or hash map layout, or helper field for the same
    data: Box<[(K, V)]>,
}

impl<K: FromJsonValue<K>, V: FromJsonValue<V>> FromJsonValue<BMap<K, V>> for BMap<K, V> {
    fn from_json_value(value: &Value) -> BMap<K, V> {
        if let Value::Object(map) = value {
            return BMap {
                length: BUInt32 {
                    data: (map.len() as u32).to_le_bytes(),
                },
                data: map
                    .iter()
                    .map(|(k, v)| {
                        (
                            K::from_json_value(&Value::String(k.clone())),
                            V::from_json_value(v),
                        )
                    })
                    .collect(),
            };
        }
        panic!("Attempting to construct BMap from non-object")
    }
}

impl<'a, K, V> ToRust<'a, &'a [(K, V)]> for BMap<K, V> {
    fn to_rust(&'a self) -> &'a [(K, V)] {
        self.data.as_ref()
    }
}
