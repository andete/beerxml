// (c) 2017 Joost Yervante Damad <joost@damad.be>

#![warn(missing_docs)]
#![recursion_limit="128"]

//! beerXML/json/yaml/toml parsing and generating library

extern crate quick_xml;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml as serde_toml;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::path::Path;

use data::RecordSet;
use error::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

/// read a beerXML, json, yaml or toml file
pub fn read_file(filename: &Path) -> Result<RecordSet> {
    if let Some(ext) = filename.extension() {
        match ext.to_str().unwrap() {
            "xml" => xml::read_file(filename),
            "json" => json::read_file(filename),
            "yaml" => yaml::read_file(filename),
            "toml" => toml::read_file(filename),
            e => Err(format!("unknown file extension {}", e).into()),
        }
    } else {
        Err("no file extension found".into())
    }
}

/// write a beerXML, json, yaml or toml file
pub fn write_file(filename: &Path, set: &RecordSet) -> Result<()> {
    if let Some(ext) = filename.extension() {
        match ext.to_str().unwrap() {
            "xml" => xml::write_file(filename, set),
            "json" => json::write_file(filename, set),
            "yaml" => yaml::write_file(filename, set),
            "toml" => toml::write_file(filename, set),
            e => Err(format!("unknown file extension {}", e).into()),
        }
    } else {
        Err("no file extension found".into())
    }
}

/// data structures
pub mod data;
/// error handling
pub mod error;
/// xml parsing and generating
pub mod xml;
/// json parsing and generating
pub mod json;
/// toml parsing and generating
pub mod toml;
/// yaml parsing and generating
pub mod yaml;
