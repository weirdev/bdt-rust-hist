use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Spec {
    #[serde(rename = "type")] 
    pub tp: SpecType,
    pub value: Value,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub enum SpecType {
    String,
    Int32,
    List(Box<SpecType>),
}

impl ToString for Spec {
    fn to_string(&self) -> String {
        Spec::view_to_string(&self.tp, &self.value)
    }
}

impl Spec {
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Spec, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let spec = serde_json::from_reader(reader)?;
        Ok(spec)
    }

    fn view_to_string(tp: &SpecType, value: &Value) -> String {
        match tp {
            SpecType::String => {
                if let Value::String(s) = value {
                    format!("\"{}\"", s.clone())
                } else {
                    panic!("SpecType::String value is not a string");
                }
            }
            SpecType::Int32 => {
                if let Value::Number(n) = value {
                    if n.is_i64() {
                        return format!("{}", n.as_i64().unwrap() as i32);
                    }
                }
                panic!("SpecType::Int32 value is not an int")
            }
            SpecType::List(inner_type) => {
                if let Value::Array(values) = value {
                    let mut result = String::from("[");
                    for inner_value in values {
                        result.push_str(&Spec::view_to_string(inner_type, inner_value));
                        result.push_str(", ");
                    }
                    result.pop();
                    result.pop();
                    result.push(']');
                    result
                } else {
                    panic!("SpecType::List value is not an array");
                }
            },
        }
    }
}
