use serde_json::Value;
use std::marker::PhantomData;
use std::mem::transmute;

use super::{FromJsonValue, ToRust};

pub struct BMap<K, V> {
    key_type: PhantomData<*const K>,
    value_type: PhantomData<*const V>,
    // TODO: Tree or hash map layout, or helper field for the same
    data: Box<[u8]>,
}

impl<K: FromJsonValue<K>, V: FromJsonValue<V>> FromJsonValue<BMap<K, V>> for BMap<K, V> {
    fn from_json_value(value: &Value) -> BMap<K, V> {
        if let Value::Object(map) = value {
            let typed_box: Box<[(K, V)]> = map
                .iter()
                .map(|(k, v)| {
                    (
                        K::from_json_value(&Value::String(k.clone())),
                        V::from_json_value(v),
                    )
                })
                .collect();
            return BMap {
                key_type: PhantomData,
                value_type: PhantomData,
                data: unsafe { transmute(typed_box) },
            };
        }
        panic!("Attempting to construct BMap from non-object")
    }
}

impl<K, V> BMap<K, V> {
    fn get_elements_as_slice(&self) -> &[(K, V)] {
        let typed_box: &Box<[(K, V)]> = unsafe { transmute(&self.data) };
        typed_box.as_ref()
    }
}

impl<'a, K, V> ToRust<'a, &'a [(K, V)]> for BMap<K, V> {
    fn to_rust(&'a self) -> &'a [(K, V)] {
        self.get_elements_as_slice()
    }
}
