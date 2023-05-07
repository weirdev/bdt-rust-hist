mod spec;
mod bint;

use bint::bstring::BString;

use spec::Spec;

use crate::bint::{FromBytes, bint32::BInt32};

fn string_test() {
    let spec = Spec::read_from_file("../json-specs/spec1.json").unwrap();
    println!("{}", spec.to_string());
    let bytes = spec.to_bytes();
    let bstring = BString::from_bytes(&bytes);
    println!("{}", bstring.to_rust());
}

fn main() {
    let spec = Spec::read_from_file("../json-specs/spec2.json").unwrap();
    println!("{}", spec.to_string());
    let bytes = spec.to_bytes();
    let bint = BInt32::from_bytes(&bytes);
    println!("{}", bint.to_rust());
}
