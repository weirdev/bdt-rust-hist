use serde_json::Value;
use std::marker::PhantomData;
use std::mem::transmute;
use std::slice;

use super::{FromJsonValue, ToRust};

pub struct BMap<K, V> {
    key_type: PhantomData<*const K>,
    value_type: PhantomData<*const V>,
    // TODO: Tree or hash map layout, or helper field for the same
    data: [u8; 16],
}

// TODO: Extern type
pub struct KVPair<K, V> {
    pub key: K,
    pub value: V,
}

impl<K: FromJsonValue<K>, V: FromJsonValue<V>> FromJsonValue<BMap<K, V>> for BMap<K, V> {
    fn from_json_value(value: &Value) -> BMap<K, V> {
        if let Value::Object(map) = value {
            let typed_box: Box<[KVPair<K, V>]> = map
                .iter()
                .map(|(k, v)| KVPair {
                    key: K::from_json_value(&Value::String(k.clone())),
                    value: V::from_json_value(v),
                })
                .collect();

            let len = map.len();

            let fat_ptr = Box::into_raw(typed_box);
            let raw_ptr = fat_ptr as *mut u8 as usize;

            return BMap {
                key_type: PhantomData,
                value_type: PhantomData,
                data: unsafe { transmute([raw_ptr, len]) },
            };
        }
        panic!("Attempting to construct BMap from non-object")
    }
}

impl<K, V> BMap<K, V> {
    fn get_elements_as_slice(&self) -> &[KVPair<K, V>] {
        let raw_ptr =
            usize::from_le_bytes(self.data[..8].try_into().unwrap()) as *const KVPair<K, V>;
        let len = usize::from_le_bytes(self.data[8..].try_into().unwrap());

        unsafe { slice::from_raw_parts(raw_ptr, len) }
    }

    fn get_elements_as_slice_mut(&mut self) -> &mut [KVPair<K, V>] {
        let raw_ptr = usize::from_le_bytes(self.data[..8].try_into().unwrap()) as *mut KVPair<K, V>;
        let len = usize::from_le_bytes(self.data[8..].try_into().unwrap());

        unsafe { slice::from_raw_parts_mut(raw_ptr, len) }
    }
}

impl<'a, K, V> ToRust<'a, &'a [KVPair<K, V>]> for BMap<K, V> {
    fn to_rust(&'a self) -> &'a [KVPair<K, V>] {
        self.get_elements_as_slice()
    }
}

impl<K, V> Drop for BMap<K, V> {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.get_elements_as_slice_mut() as *mut [KVPair<K, V>]) };
    }
}
