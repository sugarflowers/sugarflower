use crate::textreader::TextReader;
use std::collections::HashMap;
use anyhow::{Context, Result};

pub fn read(file_path: &str) -> HashMap<String, HashMap<String, String>> {
    let toml_text = TextReader::open(file_path).unwrap().read().unwrap();
    toml::from_str(&toml_text).unwrap()
}

pub fn read_check(file_path: &str) -> 
    Result< HashMap< String, HashMap< String, String > > > {

    let toml_text = TextReader::open(file_path)?.read().unwrap();
    toml::from_str(&toml_text).unwrap()
}

