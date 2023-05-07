pub mod bstring;

pub trait FromBytes<'a, T> {
    fn from_bytes(bytes: &'a [u8]) -> T;
}