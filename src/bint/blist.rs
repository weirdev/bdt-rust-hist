use serde_json::Value;
use std::marker::PhantomData;
use std::mem::transmute;

use super::{FromJsonValue, ToRust};

pub struct BList<T> {
    element_type: PhantomData<*const T>,
    data: Box<[u8]>,
}

impl<T: FromJsonValue<T>> FromJsonValue<BList<T>> for BList<T> {
    fn from_json_value(value: &serde_json::Value) -> BList<T> {
        if let Value::Array(values) = value {
            let typed_box: Box<[T]> = values.iter().map(|v| T::from_json_value(v)).collect();
            return BList {
                element_type: PhantomData,
                data: unsafe { transmute(typed_box) },
            };
        }
        panic!("Attempting to construct BList from non-array")
    }
}

impl<'a, T> BList<T> {
    fn get_elements_as_slice(&'a self) -> &'a [T] {
        let typed_box: &Box<[T]> = unsafe { transmute(&self.data) };
        typed_box.as_ref()
    }
}

impl<'a, T> ToRust<'a, &'a [T]> for BList<T> {
    fn to_rust(&'a self) -> &'a [T] {
        self.get_elements_as_slice()
    }
}
