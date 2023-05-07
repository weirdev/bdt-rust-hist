use serde_json::Value;

use super::FromJsonValue;

pub struct BInt32 {
    // Stored in little endian
    // Four bytes are used explicitly, rather than i32, to ensure wire consistency
    data: [u8; 4],
}

impl FromJsonValue<BInt32> for BInt32 {
    fn from_json_value(value: &serde_json::Value) -> BInt32 {
        if let Value::Number(n) = value {
            if n.is_i64() {
                return BInt32 {
                    data: (n.as_i64().unwrap() as i32).to_le_bytes(),
                };
            }
        }
        panic!("SpecType::Int32 value is not an int")  
    }
}

impl BInt32 {
    #[inline]
    pub fn to_rust(&self) -> i32 {
        i32::from_le_bytes(self.data)
    }
}