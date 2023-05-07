pub mod bstring;
pub mod bint32;

pub trait FromBytes<'a, T> {
    fn from_bytes(bytes: &'a [u8]) -> T;
}