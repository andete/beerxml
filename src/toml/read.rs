// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use serde_toml;

use data::*;
use error::*;

#[derive(Deserialize)]
struct PrivateDocument {
    document: String,
}

fn read_str(s: &str) -> Result<RecordSet> {
    let pd: PrivateDocument = serde_toml::from_str(s)?;
    let document = pd.document.as_str();
    let marker = format!("document = \"{}\"", document);
    let s2 = s.replacen(&marker, "", 1);
    match document {
        "Fermentables" => {
            let h: HashMap<String, Fermentable> = serde_toml::from_str(&s2)?;
            Ok(RecordSet::Fermentables(h))
        }
        "Hops" => {
            let h: HashMap<String, Hop> = serde_toml::from_str(&s2)?;
            Ok(RecordSet::Hops(h))
        }
        "Miscs" => {
            let h: HashMap<String, Misc> = serde_toml::from_str(&s2)?;
            Ok(RecordSet::Miscs(h))
        }
        "Waters" => {
            let h: HashMap<String, Water> = serde_toml::from_str(&s2)?;
            Ok(RecordSet::Waters(h))
        }
        "Yeasts" => {
            let h: HashMap<String, Yeast> = serde_toml::from_str(&s2)?;
            Ok(RecordSet::Yeasts(h))
        }
        "Recipes" => {
            let h: HashMap<String, Recipe> = serde_toml::from_str(&s2)?;
            Ok(RecordSet::Recipes(h))
        }
        e => Err(format!("Toml: Unimplemented document type {}", e).into()),
    }
}

/// try to read a `RecordSet` from a toml file
pub fn read_file(filename: &Path) -> Result<RecordSet> {
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    read_str(&contents)
}
