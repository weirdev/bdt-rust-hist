use super::FromBytes;

pub struct BInt32<'a> {
    data: &'a [u8],
}

impl<'a> FromBytes<'a, BInt32<'a>> for BInt32<'a> {
    fn from_bytes(bytes: &'a [u8]) -> BInt32<'a> {
        BInt32 { data: bytes }
    }
}

impl BInt32<'_> {
    #[inline]
    pub fn to_rust(&self) -> i32 {
        i32::from_le_bytes(self.data.try_into().unwrap())
    }
}