mod spec;
mod bint;

use bint::bstring::BString;

use spec::Spec;

use crate::bint::FromBytes;

fn main() {
    let spec = Spec::read_from_file("../json-specs/spec1.json").unwrap();
    println!("{}", spec.to_string());
    let bytes = spec.to_bytes();
    let bstring = BString::from_bytes(&bytes);
    println!("{}", bstring.to_rust());
}
