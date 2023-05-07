use serde::Deserialize;
use serde::Serialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Spec {
    String(String)
}

impl ToString for Spec {
    fn to_string(&self) -> String {
        match self {
            Spec::String(s) => format!("\"{}\"", s.clone())
        }
    }
}

impl Spec {
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Spec, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let spec = serde_json::from_reader(reader)?;
        Ok(spec)
    }

    pub fn to_bytes(&self) -> Box<[u8]> {
        match self {
            Spec::String(s) => s.as_bytes().iter().cloned().collect()
        }
    }
}