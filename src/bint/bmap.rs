use serde_json::Value;
use std::marker::PhantomData;
use std::mem::transmute;
use std::slice;

use super::{FromJsonValue, ToRust};

#[repr(transparent)]
pub struct BMap<K, V>
where
    [(); std::mem::size_of::<K>() + std::mem::size_of::<V>()]: Sized,
{
    key_type: PhantomData<*const K>,
    value_type: PhantomData<*const V>,
    // TODO: Tree or hash map layout, or helper field for the same
    data: [u8; 16],
}

#[repr(transparent)]
pub struct KVPair<K, V>
where
    [(); std::mem::size_of::<K>() + std::mem::size_of::<V>()]: Sized,
{
    key_type: PhantomData<*const K>,
    value_type: PhantomData<*const V>,
    data: [u8; std::mem::size_of::<K>() + std::mem::size_of::<V>()],
}

impl<'a, K, V> ToRust<'a, (&'a K, &'a V)> for KVPair<K, V>
where
    [(); std::mem::size_of::<K>() + std::mem::size_of::<V>()]: Sized,
{
    fn to_rust(&'a self) -> (&'a K, &'a V) {
        let key_ptr = (&self.data[0] as *const u8) as *const K;
        let value_ptr = (&self.data[std::mem::size_of::<K>()] as *const u8) as *const V;

        unsafe { (key_ptr.as_ref().unwrap(), value_ptr.as_ref().unwrap()) }
    }
}

impl<K, V> Drop for KVPair<K, V>
where
    [(); std::mem::size_of::<K>() + std::mem::size_of::<V>()]: Sized,
{
    fn drop(&mut self) {
        let key_ptr = (&mut self.data[0] as *mut u8) as *mut K;
        let value_ptr = (&mut self.data[std::mem::size_of::<K>()] as *mut u8) as *mut V;

        unsafe {
            key_ptr.drop_in_place();
            value_ptr.drop_in_place();
        }
    }
}

impl<K, V> KVPair<K, V>
where
    [(); std::mem::size_of::<K>() + std::mem::size_of::<V>()]: Sized,
{
    fn new(key: K, value: V) -> KVPair<K, V> {
        let mut pair = KVPair {
            key_type: PhantomData,
            value_type: PhantomData,
            // TODO: Fill with uninit
            data: [0; std::mem::size_of::<K>() + std::mem::size_of::<V>()],
        };

        let key_ptr = (&mut pair.data[0] as *mut u8) as *mut K;
        let value_ptr = (&mut pair.data[std::mem::size_of::<K>()] as *mut u8) as *mut V;

        unsafe {
            key_ptr.write_unaligned(key);
            value_ptr.write_unaligned(value);
        }

        pair
    }
}

impl<K: FromJsonValue<K>, V: FromJsonValue<V>> FromJsonValue<BMap<K, V>> for BMap<K, V>
where
    [(); std::mem::size_of::<K>() + std::mem::size_of::<V>()]: Sized,
{
    fn from_json_value(value: &Value) -> BMap<K, V> {
        if let Value::Object(map) = value {
            let len = map.len();

            let typed_box: Box<[KVPair<K, V>]> = map
                .iter()
                .map(|(k, v)| {
                    KVPair::new(
                        K::from_json_value(&Value::String(k.clone())),
                        V::from_json_value(v),
                    )
                })
                .collect();

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

impl<K, V> BMap<K, V>
where
    [(); std::mem::size_of::<K>() + std::mem::size_of::<V>()]: Sized,
{
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

impl<'a, K, V> ToRust<'a, &'a [KVPair<K, V>]> for BMap<K, V>
where
    [(); std::mem::size_of::<K>() + std::mem::size_of::<V>()]: Sized,
{
    fn to_rust(&'a self) -> &'a [KVPair<K, V>] {
        self.get_elements_as_slice()
    }
}

impl<K, V> Drop for BMap<K, V>
where
    [(); std::mem::size_of::<K>() + std::mem::size_of::<V>()]: Sized,
{
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.get_elements_as_slice_mut() as *mut [KVPair<K, V>]) };
    }
}
