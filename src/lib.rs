// (c) 2017 Joost Yervante Damad <joost@damad.be>

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

pub fn read_file(filename: &Path) -> Result<RecordSet> {
    if let Some(ext) = filename.extension() {
        match ext.to_str().unwrap() {
            "xml" => xml::read_file(filename),
            // "json" => json::read_file(filename),
            // "yaml" => yaml::read_file(filename),
            // "toml" => toml::read_file(filename),
            e => Err(format!("unknown file extension {}", e).into()),
        }
    } else {
        Err("no file extension found".into())
    }
}

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

pub mod data;
pub mod error;
pub mod xml;
pub mod json;
pub mod toml;
pub mod yaml;
