// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde_toml;

use data::*;
use error::*;

pub fn write<T>(writer: &mut T, set: &RecordSet) -> Result<()>
    where T: Write
{
    let s = match *set {
        RecordSet::Empty => return Ok(()),
        RecordSet::Fermentables(ref v) => {
            write!(writer, "document = \"Fermentables\"\n\n")?;
            serde_toml::to_string(v)?
        }
        RecordSet::Hops(ref v) => {
            write!(writer, "document = \"Hops\"\n\n")?;
            serde_toml::to_string(v)?
        }
        RecordSet::Yeasts(ref v) => {
            write!(writer, "document = \"Yeasts\"\n\n")?;
            serde_toml::to_string(v)?
        }
        RecordSet::Miscs(ref v) => {
            write!(writer, "document = \"Miscs\"\n\n")?;
            serde_toml::to_string(v)?
        }
        RecordSet::Waters(ref v) => {
            write!(writer, "document = \"Waters\"\n\n")?;
            serde_toml::to_string(v)?
        }
    };
    write!(writer, "{}", s)?;
    Ok(())
}

pub fn write_file(filename: &Path, set: &RecordSet) -> Result<()> {
    let mut f = File::create(filename)?;
    write(&mut f, set)
}
