use crate::textreader::TextReader;
use std::collections::HashMap;

pub fn read(file_path: &str) -> HashMap<String, HashMap<String, String>> {
    let toml_text = TextReader::open(file_path).unwrap().read().unwrap();
    toml::from_str(&toml_text).unwrap()
}

