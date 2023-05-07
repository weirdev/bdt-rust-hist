use serde_json::Value;

use super::{FromJsonValue, ToRust};

#[repr(transparent)]
pub struct BUInt32 {
    // Stored in little endian
    // Four bytes are used explicitly, rather than u32, to ensure wire consistency
    pub data: [u8; 4],
}

impl FromJsonValue<BUInt32> for BUInt32 {
    fn from_json_value(value: &serde_json::Value) -> BUInt32 {
        if let Value::Number(n) = value {
            if n.is_u64() {
                return BUInt32 {
                    data: (n.as_u64().unwrap() as u32).to_le_bytes(),
                };
            }
        }
        panic!("Attempting to create BInt32 from non-int")
    }
}

impl ToRust<'_, u32> for BUInt32 {
    fn to_rust(&self) -> u32 {
        u32::from_le_bytes(self.data)
    }
}
