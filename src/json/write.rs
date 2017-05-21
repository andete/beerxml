// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde_json;

use data::*;
use error::*;

pub fn write<T>(writer: &mut T, set: &RecordSet) -> Result<()>
    where T: Write
{
    match *set {
        RecordSet::Empty => (),
        RecordSet::Fermentables(ref v) => serde_json::to_writer_pretty(writer, v)?,
    }
    Ok(())
}

pub fn write_file(filename: &Path, set: &RecordSet) -> Result<()> {
    let mut f = File::create(filename)?;
    write(&mut f, set)
}
