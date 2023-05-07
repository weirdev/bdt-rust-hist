mod spec;
mod bint;

use bint::bstring::BString;

use spec::Spec;

use crate::bint::{bint32::BInt32, FromJsonValue};

fn string_test() {
    let spec = Spec::read_from_file("../json-specs/spec1.json").unwrap();
    println!("{}", spec.to_string());
    let bstring = BString::from_json_value(&spec.value);
    println!("{}", bstring.to_rust());
}

fn main() {
    string_test();
    let spec = Spec::read_from_file("../json-specs/spec2.json").unwrap();
    println!("{}", spec.to_string());
    let bint = BInt32::from_json_value(&spec.value);
    println!("{}", bint.to_rust());
}
