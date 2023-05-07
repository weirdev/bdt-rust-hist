use std::str;

use super::FromBytes;

pub struct BString<'a> {
    data: &'a [u8],
}

impl<'a> FromBytes<'a, BString<'a>> for BString<'a> {
    fn from_bytes(bytes: &'a [u8]) -> BString<'a> {
        BString { data: bytes }
    }
}

impl BString<'_> {
    #[inline]
    pub fn to_rust(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.data) }
    }
}
