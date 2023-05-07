mod bint;
mod spec;

use bint::bstring::BString;

use spec::Spec;

use crate::bint::{bint32::BInt32, blist::BList, bmap::BMap, FromJsonValue, ToRust};

fn string_test() {
    let spec = Spec::read_from_file("../json-specs/spec1.json").unwrap();
    println!("{}", spec.to_string());
    let bstring = BString::from_json_value(&spec.value);
    println!("{}", bstring.to_rust());
}

fn int32_test() {
    let spec = Spec::read_from_file("../json-specs/spec2.json").unwrap();
    println!("{}", spec.to_string());
    let bint = BInt32::from_json_value(&spec.value);
    println!("{}", bint.to_rust());
}

fn list_int32_test() {
    let spec = Spec::read_from_file("../json-specs/spec3.json").unwrap();
    println!("{}", spec.to_string());
    let bint = BList::<BInt32>::from_json_value(&spec.value);
    println!(
        "{}",
        &bint
            .to_rust()
            .iter()
            .map(|v| format!("{}", v.to_rust()))
            .fold(String::new(), |a, b| a + "," + &b)
            .as_str()[1..]
    );
}

fn list_string_test() {
    let spec = Spec::read_from_file("../json-specs/spec4.json").unwrap();
    println!("{}", spec.to_string());
    let blist = BList::<BString>::from_json_value(&spec.value);
    println!(
        "{}",
        &blist
            .to_rust()
            .iter()
            .map(|v| format!("{}", v.to_rust()))
            .fold(String::new(), |a, b| a + "," + &b)
            .as_str()[1..]
    );
}

fn map_string_int32_test() {
    let spec = Spec::read_from_file("../json-specs/spec5.json").unwrap();
    println!("{}", spec.to_string());
    let bint = BMap::<BString, BInt32>::from_json_value(&spec.value);
    println!(
        "{}",
        &bint
            .to_rust()
            .iter()
            .map(|(k, v)| format!("{}:{}", k.to_rust(), v.to_rust()))
            .fold(String::new(), |a, b| a + "," + &b)
            .as_str()[1..]
    );
}

fn main() {
    string_test();
    int32_test();
    list_int32_test();
    list_string_test();
    map_string_int32_test();
}
