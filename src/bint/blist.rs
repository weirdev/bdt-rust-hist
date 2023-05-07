use serde_json::Value;
use std::marker::PhantomData;
use std::mem::transmute;
use std::slice;

use super::{FromJsonValue, ToRust};

pub struct BList<T> {
    element_type: PhantomData<*const T>,
    data: [u8; 16],
}

impl<T: FromJsonValue<T>> FromJsonValue<BList<T>> for BList<T> {
    fn from_json_value(value: &serde_json::Value) -> BList<T> {
        if let Value::Array(values) = value {
            let typed_box: Box<[T]> = values.iter().map(|v| T::from_json_value(v)).collect();

            let len = values.len();

            let fat_ptr = Box::into_raw(typed_box);
            let raw_ptr = fat_ptr as *mut u8 as usize;

            return BList {
                element_type: PhantomData,
                data: unsafe { transmute([raw_ptr, len]) },
            };
        }
        panic!("Attempting to construct BList from non-array")
    }
}

impl<'a, T> BList<T> {
    fn get_elements_as_slice(&'a self) -> &'a [T] {
        let raw_ptr = usize::from_le_bytes(self.data[..8].try_into().unwrap()) as *const T;
        let len = usize::from_le_bytes(self.data[8..].try_into().unwrap());

        // println!("raw_ptr: {:p}", raw_ptr);
        // println!("len: {}", len);

        unsafe { slice::from_raw_parts(raw_ptr, len) }
    }

    fn get_elements_as_slice_mut(&'a mut self) -> &'a mut [T] {
        let raw_ptr = usize::from_le_bytes(self.data[..8].try_into().unwrap()) as *mut T;
        let len = usize::from_le_bytes(self.data[8..].try_into().unwrap());

        // println!("raw_ptr: {:p}", raw_ptr);
        // println!("len: {}", len);

        unsafe { slice::from_raw_parts_mut(raw_ptr, len) }
    }
}

impl<'a, T> ToRust<'a, &'a [T]> for BList<T> {
    fn to_rust(&'a self) -> &'a [T] {
        self.get_elements_as_slice()
    }
}

impl<T> Drop for BList<T> {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.get_elements_as_slice_mut() as *mut [T]) };
    }
}
